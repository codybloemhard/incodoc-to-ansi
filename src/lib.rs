use incodoc::*;

use std::mem;

use zen_colour::*;
use bat::PrettyPrinter;

use term_table::*;
use term_table::row::Row;
use term_table::table_cell::TableCell;

#[derive(Clone, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Context {
    pub ps: ParStatus,
    pub modifier: String,
    pub modifier_stack: Vec<String>,
    pub indentation: usize,
    pub indented: usize,
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

    pub fn indent(&self, indentation_addition: usize, indented: usize) -> Self {
        let mut child = self.clone();
        child.indented = indented + child.indentation;
        child.indentation += indentation_addition;
        child
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
                paragraph_to_ansi(par, &mut c.indent(2, 0), output);
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
            ParagraphItem::List(list) => {
                list_to_ansi(list, c, output);
            },
            ParagraphItem::Table(table) => {
                table_to_ansi(table, c, output);
            },
        }
    }
}

pub fn list_to_ansi(list: &List, c: &mut Context, output: &mut String) {
    for par in &list.items {
        if c.ps != ParStatus::Newline {
            *output += "\n";
            c.ps = ParStatus::Newline;
        }
        indent(0, c, output);
        *output += "- ";
        c.ps = ParStatus::New;
        paragraph_to_ansi(par, &mut c.indent(2, 2), output);
        c.ps = ParStatus::Element;
    }
}

pub fn table_to_ansi(table: &incodoc::Table, c: &mut Context, output: &mut String) {
    let mut max = 0;
    for row in &table.rows {
        max = max.max(row.items.len());
    }
    let mut t = term_table::Table::builder()
        .style(TableStyle::thin())
        .build();
    for row in &table.rows {
        let mut r = Row::empty();
        for item in &row.items {
            let mut temp = String::new();
            paragraph_to_ansi(item, &mut Context::default(), &mut temp);
            r.add_cell(TableCell::new(temp));
        }
        t.add_row(r);
    }

    let mut indent_string_0 = String::new();
    indent_string_0 += "\n";
    indent(0, c, &mut indent_string_0);
    let mut indent_string_1 = String::new();
    indent_string_1 += "\n";
    indent(0, c, &mut indent_string_1);
    let mut res = String::new();
    res += &indent_string_0[1..];
    res += &t.render();
    res = res.replace("\n", &indent_string_1);
    for _ in 0..indent_string_1.len() {
        res.pop();
    }

    if c.ps != ParStatus::Newline && c.ps != ParStatus::New {
        *output += "\n";
        c.ps = ParStatus::Newline;
    }
    *output += RESET;
    *output += &res;
    *output += &c.modifier;
    c.ps = ParStatus::Element;
}

pub fn code_to_ansi(
    code: &Result<CodeBlock, CodeIdentError>,
    c: &mut Context,
    output: &mut String
) {
    let mut temp = String::new();
    let mut indent_string = String::new();
    indent_string += "\n";
    indent(2, c, &mut indent_string);
    temp += &indent_string[1..];

    match code {
        Ok(code) => {
            let res = PrettyPrinter::new()
                .input_from_bytes(code.code.as_bytes())
                .language(&code.language)
                .theme("ansi")
                .print_with_writer(Some(&mut temp));
            match res {
                Ok(true) => { },
                Ok(false) => {
                    temp += "error: bat couldn't render code\n";
                    temp += &code.code;
                },
                Err(error) => {
                    temp += "error: bat error: ";
                    temp += &format!("{error}\n");
                    temp += &code.code;
                },
            }
        },
        Err(_) => {
            temp += "error: incodoc code identation error";
        },
    }


    let mut indent_string = String::new();
    indent_string += "\n";
    indent(2, c, &mut indent_string);
    temp = temp.replace("\n", &indent_string);
    temp = temp.trim_end().to_string();

    if c.ps != ParStatus::Newline && c.ps != ParStatus::New {
        *output += "\n";
        c.ps = ParStatus::Newline;
    }
    *output += RESET;
    *output += &temp;
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
        ParStatus::Element => {
            res.push('\n');
            c.ps = ParStatus::Newline;
        },
        _ => { },
    }
    for ch in text.chars() {
        if c.ps == ParStatus::Newline || c.ps == ParStatus::New {
            indent(0, c, &mut res);
        }
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

pub fn indent(extra: usize, c: &mut Context, output: &mut String) {
    let indentation = c.indentation + extra;
    for _ in 0..(indentation - c.indented.min(indentation)) {
        *output += " ";
    }
    c.indented = 0;
}

