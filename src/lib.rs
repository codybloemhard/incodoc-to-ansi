use incodoc::*;

use std::mem;

use zen_colour::*;
use bat::PrettyPrinter;

#[derive(Clone, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Context {
    pub ps: ParStatus,
    pub modifier: String,
    pub modifier_stack: Vec<String>,
}

impl Context {
    pub fn push_parstat(&mut self, new: &str, output: &mut String) {
        self.modifier_stack.push(mem::take(&mut self.modifier));
        self.modifier = new.to_string();
        *output += &self.modifier;
    }

    pub fn pop_parstat(&mut self, output: &mut String) {
        *output += RESET;
        self.modifier = self.modifier_stack.pop().unwrap_or_default();
        *output += &self.modifier;
    }
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ParStatus {
    /// new paragraph has started
    #[default]
    New,
    Newline,
    /// whitespace other than a new line
    Whitespace,
    /// regular character
    Char,
    /// non-text element: code, list, table, image, etc
    Element,
    Emphasis,
}

/// Take an incodoc and unparse it to ANSI.
/// Use just this function unless doing something fancy.
pub fn doc_to_ansi_string(doc: &Doc) -> String {
    let mut res = String::new();
    let mut context = Context {
        modifier: RESET.to_string(),
        ..Default::default()
    };
    doc_to_ansi(doc, &mut context, &mut res);
    res
}

pub fn doc_to_ansi(doc: &Doc, c: &mut Context, output: &mut String) {
    for item in &doc.items {
        match item {
            DocItem::Nav(nav) => {},
            DocItem::Paragraph(par) => {
                c.ps = ParStatus::New;
                paragraph_to_ansi(par, c, output);
            },
            DocItem::Section(section) => {},
        }
        *output += "\n\n";
    }
    output.pop();
    output.pop();
}

pub fn paragraph_to_ansi(par: &Paragraph, c: &mut Context, output: &mut String) {
    for item in &par.items {
        match item {
            ParagraphItem::Text(text) => {
                text_to_ansi(text, c, output);
            },
            ParagraphItem::MText(TextWithMeta { text, tags, props }) => {
                text_to_ansi(text, c, output);
            },
            ParagraphItem::Em(emphasis) => {
                emphasis_to_ansi(emphasis, c, output);
            },
            ParagraphItem::Link(link) => {
                link_to_ansi(link, c, output);
            },
            ParagraphItem::Code(code) => {
                code_to_ansi(code, c, output);
            },
            ParagraphItem::List(list) => {},
            ParagraphItem::Table(table) => {},
        }
    }
}

pub fn code_to_ansi(
    code: &Result<CodeBlock, CodeIdentError>,
    c: &mut Context,
    output: &mut String
) {
    *output += RESET;
    match code {
        Ok(code) => {
            let mut temp = String::new();
            let res = PrettyPrinter::new()
                .input_from_bytes(code.code.as_bytes())
                .language(&code.language)
                .theme("ansi")
                .print_with_writer(Some(&mut temp));
            match res {
                Ok(true) => *output += &temp,
                Ok(false) => {
                    *output += "error: bat couldn't render code\n";
                    *output += &code.code;
                },
                Err(error) => {
                    *output += "error: bat error: ";
                    *output += &format!("{error}\n");
                    *output += &code.code;
                },
            }
        },
        Err(_) => {
            *output += "error: incodoc code identation error";
        },
    }
    *output += "\n";
    *output += &c.modifier;
    c.ps = ParStatus::Element;
}

pub fn link_to_ansi(link: &Link, c: &mut Context, output: &mut String) {
    c.push_parstat(MAGENTA, output);
    for item in &link.items {
        match item {
            LinkItem::String(text) => text_to_ansi(text, c, output),
            LinkItem::Em(em) => emphasis_to_ansi(em, c, output),
        }
    }
    c.pop_parstat(output);
}

pub fn emphasis_to_ansi(em: &Emphasis, c: &mut Context, output: &mut String) {
    let modifier = match (em.etype, em.strength) {
        (EmType::Emphasis, EmStrength::Light) => ITALIC.to_string(),
        (EmType::Emphasis, EmStrength::Medium) => BOLD.to_string(),
        (EmType::Emphasis, EmStrength::Strong) => format!("{ITALIC}{BOLD}"),
        (EmType::Deemphasis, EmStrength::Light) => FAINT.to_string(),
        (EmType::Deemphasis, EmStrength::Medium) => CROSSED.to_string(),
        (EmType::Deemphasis, EmStrength::Strong) => HIDDEN.to_string(),
    };
    *output += &modifier;
    *output += &format_text(&em.text, c);
    *output += RESET;
    *output += &c.modifier;
    c.ps = ParStatus::Emphasis;
}

pub fn text_to_ansi(text: &str, c: &mut Context, output: &mut String) {
    *output += &format_text(text, c);
}

pub fn format_text(text: &str, c: &mut Context) -> String {
    let mut res = String::new();
    match c.ps {
        ParStatus::Char => res.push(' '),
        ParStatus::Element => res.push('\n'),
        _ => { },
    }
    for ch in text.chars() {
        match ch {
            '\n' => {
                if c.ps == ParStatus::Whitespace || c.ps == ParStatus::Char {
                    c.ps = ParStatus::Newline;
                    res.push('\n');
                }
            },
            '\r' => {},
            x => {
                if x.is_whitespace() {
                    if c.ps != ParStatus::Whitespace {
                        if c.ps != ParStatus::Newline {
                            res.push(' ');
                        }
                        c.ps = ParStatus::Whitespace;
                    }
                } else {
                    c.ps = ParStatus::Char;
                    res.push(x);
                }
            },
        }
    }
    res
}
