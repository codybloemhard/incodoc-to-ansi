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
    pub ii_stack: Vec<(usize, usize)>,
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

    pub fn push_indent(&mut self, indentation_addition: usize, indented: usize) {
        self.ii_stack.push((self.indentation, self.indented));
        if indented > 0 {
            self.indented = indented + self.indentation;
        }
        self.indentation += indentation_addition;
    }

    pub fn pop_indent(&mut self) {
        let (old_indentation, old_indented) = self.ii_stack.pop().expect("could not pop ii_stack");
        self.indentation = old_indentation;
        self.indented = old_indented;
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
            DocItem::Nav(nav) => nav_to_ansi(nav, c, output),
            DocItem::Paragraph(par) => {
                c.ps = ParStatus::New;
                paragraph_to_ansi(par, c, output);
            },
            DocItem::Section(section) => section_to_ansi(section, c, output),
        }
        *output += "\n\n";
    }
    output.pop();
    output.pop();
}

pub fn nav_to_ansi(nav: &Nav, c: &mut Context, output: &mut String) {
    newline_not(&[ParStatus::Newline], c, output);
    text_to_ansi(&nav.description, c, output);
    newline(c, output);

    for link in &nav.links {
        newline_not(&[ParStatus::Newline], c, output);
        c.push_indent(2, 0);
        link_to_ansi(link, c, output);
        c.pop_indent();
    }

    for sub in &nav.subs {
        c.push_indent(2, 0);
        nav_to_ansi(sub, c, output);
        c.pop_indent();
    }
}

pub fn section_to_ansi(section: &Section, c: &mut Context, output: &mut String) {
    c.ps = ParStatus::New;
    heading_to_ansi(&section.heading, c, output);
    newline(c, output);
    for item in &section.items {
        match item {
            SectionItem::Paragraph(par) => {
                c.ps = ParStatus::New;
                c.push_indent(2, 0);
                paragraph_to_ansi(par, c, output);
                c.pop_indent();
            },
            SectionItem::Section(section) => {
                c.push_indent(2, 0);
                section_to_ansi(section, c, output);
                c.pop_indent();
            },
        }
        *output += "\n\n";
    }
    output.pop();
    output.pop();
}

pub fn heading_to_ansi(heading: &Heading, c: &mut Context, output: &mut String) {
    c.push_parstat(BOLD, output);
    for item in &heading.items {
        match item {
            HeadingItem::String(string) => text_to_ansi(string, c, output),
            HeadingItem::Em(emphasis) => emphasis_to_ansi(emphasis, c, output),
        }
    }
    c.pop_parstat(output);
}

pub fn paragraph_to_ansi(par: &Paragraph, c: &mut Context, output: &mut String) {
    for item in &par.items {
        match item {
            ParagraphItem::Text(text) => {
                text_to_ansi(text, c, output);
            },
            ParagraphItem::MText(TextWithMeta { text, tags, .. }) => {
                if tags.contains("code") {
                    inline_code_to_ansi(text, c, output);
                } else {
                    text_to_ansi(text, c, output);
                }
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
    let width = match list.ltype {
        ListType::Distinct => format!("{}", list.items.len().max(1) - 1).len(),
        ListType::Identical => 2,
        ListType::Checked => 6,
    };
    let iwidth = match list.ltype {
        ListType::Distinct => width + 2,
        ListType::Identical | ListType::Checked => width,
    };
    for (count, par) in list.items.iter().enumerate() {
        newline_not(&[ParStatus::Newline], c, output);
        indent(0, c, output);
        match list.ltype {
            ListType::Distinct => *output += &format!("{count:>width$}. "),
            ListType::Identical => *output += "- ",
            ListType::Checked if par.tags.contains("checked") => *output += "- [x] ",
            ListType::Checked => *output += "- [ ] ",
        }
        c.ps = ParStatus::New;
        c.push_indent(iwidth, iwidth);
        paragraph_to_ansi(par, c, output);
        c.pop_indent();
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

    newline_not(&[ParStatus::Newline, ParStatus::New], c, output);
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

    newline_not(&[ParStatus::Newline, ParStatus::New], c, output);
    *output += RESET;
    *output += &temp;
    *output += &c.modifier;
    c.ps = ParStatus::Element;
}

pub fn inline_code_to_ansi(text: &str, c: &mut Context, output: &mut String) {
    format_text_pre(c, output);
    *output += RESET;
    *output += BG_BLACK;
    format_text_main(text, c, output);
    *output += RESET;
    *output += &c.modifier;
    c.ps = ParStatus::Char;
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
    format_text(&em.text, c, output);
    *output += RESET;
    *output += &c.modifier;
    c.ps = ParStatus::Emphasis;
}

pub fn text_to_ansi(text: &str, c: &mut Context, output: &mut String) {
    format_text(text, c, output);
}

pub fn format_text(text: &str, c: &mut Context, output: &mut String) {
    format_text_pre(c, output);
    format_text_main(text, c, output);
}

pub fn format_text_pre(c: &mut Context, output: &mut String) {
    match c.ps {
        ParStatus::Char => {
            output.push(' ');
            c.ps = ParStatus::Whitespace;
        },
        ParStatus::Element => {
            output.push('\n');
            c.ps = ParStatus::Newline;
        },
        _ => { },
    }
}

pub fn format_text_main(text: &str, c: &mut Context, output: &mut String) {
    for ch in text.chars() {
        if c.ps == ParStatus::Newline || c.ps == ParStatus::New {
            indent(0, c, output);
        }
        match ch {
            '\n' => {
                if c.ps == ParStatus::Whitespace || c.ps == ParStatus::Char {
                    c.ps = ParStatus::Newline;
                    output.push('\n');
                }
            },
            '\r' => {},
            x => {
                if x.is_whitespace() {
                    if c.ps != ParStatus::Whitespace {
                        if c.ps != ParStatus::Newline {
                            output.push(' ');
                        }
                        c.ps = ParStatus::Whitespace;
                    }
                } else {
                    c.ps = ParStatus::Char;
                    output.push(x);
                }
            },
        }
    }
}

pub fn indent(extra: usize, c: &mut Context, output: &mut String) {
    let indentation = c.indentation + extra;
    for _ in 0..(indentation - c.indented.min(indentation)) {
        *output += " ";
    }
    c.indented = 0;
}

pub fn newline(c: &mut Context, output: &mut String) {
    *output += "\n";
    c.ps = ParStatus::Newline;
}

pub fn newline_not(pss: &[ParStatus], c: &mut Context, output: &mut String) {
    for ps in pss {
        if c.ps == *ps {
            return;
        }
    }
    *output += "\n";
    c.ps = ParStatus::Newline;
}
