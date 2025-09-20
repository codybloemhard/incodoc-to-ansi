use incodoc::*;

use zen_colour::*;

use std::fmt::Write;

#[derive(Clone, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Context {
    pub ps: ParStatus,
    pub modifier: String,
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
    }
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
            ParagraphItem::Link(link) => {},
            ParagraphItem::Code(code) => {},
            ParagraphItem::List(list) => {},
            ParagraphItem::Table(table) => {},
        }
    }
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
    let _ = write!(output, "{modifier}{}{}", format_text(&em.text, c), c.modifier);
}

pub fn text_to_ansi(text: &str, c: &mut Context, output: &mut String) {
    let _ = write!(output, "{}", format_text(text, c));
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
