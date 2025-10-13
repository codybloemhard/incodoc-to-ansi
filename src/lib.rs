use incodoc::*;

use std::mem;

use zen_colour::*;
use bat::{ PrettyPrinter, WrappingMode};

use term_table::*;
use term_table::row::Row;
use term_table::table_cell::TableCell;

#[derive(Clone, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Context {
    pub ps: ParStatus,
    pub fg_mod: String,
    pub bg_mod: String,
    pub fg_mod_stack: Vec<String>,
    pub bg_mod_stack: Vec<String>,
    pub ii_stack: Vec<(usize, usize)>,
    pub indentation: usize,
    pub indented: usize,
    pub width: usize,
    pub col: usize,
}

impl Context {
    pub fn push_fg_mod(&mut self, new: &str, output: &mut String) {
        self.fg_mod_stack.push(mem::take(&mut self.fg_mod));
        self.fg_mod = new.to_string();
        *output += &self.fg_mod;
    }

    pub fn pop_fg_mod(&mut self, output: &mut String) {
        *output += RESET;
        self.fg_mod = self.fg_mod_stack.pop().unwrap_or_default();
        *output += &self.fg_mod;
        *output += &self.bg_mod;
    }

    pub fn push_bg_mod(&mut self, new: &str, output: &mut String) {
        self.bg_mod_stack.push(mem::take(&mut self.bg_mod));
        self.bg_mod = new.to_string();
        *output += &self.bg_mod;
    }

    pub fn pop_bg_mod(&mut self, output: &mut String) {
        *output += RESET;
        self.bg_mod = self.bg_mod_stack.pop().unwrap_or_default();
        *output += &self.bg_mod;
        *output += &self.fg_mod;
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
    Newline(usize),
    /// whitespace other than a new line
    Whitespace,
    /// regular character
    Char,
    /// non-text element: code, list, table, image, etc
    Element,
    Emphasis,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Config {
    pub width: usize,
}

/// Take an incodoc and unparse it to ANSI.
/// Use just this function unless doing something fancy.
pub fn doc_to_ansi_string(doc: &Doc, conf: &Config) -> String {
    let mut res = String::new();
    let mut context = Context {
        fg_mod: RESET.to_string(),
        width: conf.width,
        ..Default::default()
    };
    doc_to_ansi(doc, conf, &mut context, &mut res);
    res
}

pub fn doc_to_ansi(doc: &Doc, conf: &Config, c: &mut Context, output: &mut String) {
    for item in &doc.items {
        match item {
            DocItem::Nav(nav) => nav_to_ansi(nav, conf, c, output),
            DocItem::Paragraph(par) => {
                c.ps = ParStatus::New;
                paragraph_to_ansi(par, conf, c, output);
            },
            DocItem::Section(section) => section_to_ansi(section, conf, c, output),
        }
    }
}

pub fn nav_to_ansi(nav: &Nav, conf: &Config, c: &mut Context, output: &mut String) {
    newlines_unless(1, &[], c, output);
    text_to_ansi(&nav.description, c, output);
    newline(c, output);

    for link in &nav.links {
        newlines_unless(1, &[], c, output);
        c.push_indent(2, 0);
        link_to_ansi(link, conf, c, output);
        c.pop_indent();
    }

    for sub in &nav.subs {
        c.push_indent(2, 0);
        nav_to_ansi(sub, conf, c, output);
        c.pop_indent();
    }
}

pub fn section_to_ansi(section: &Section, conf: &Config, c: &mut Context, output: &mut String) {
    if section.tags.contains("blockquote") || section.tags.contains("blockquote-typed") {
        blockquote_to_ansi(section, conf, c, output);
    } else {
        headed_section_to_ansi(section, conf, c, output);
    }
}

pub fn headed_section_to_ansi(
    section: &Section, conf: &Config, c: &mut Context, output: &mut String
) {
    c.ps = ParStatus::New;
    newlines_unless(1, &[], c, output);
    heading_to_ansi(&section.heading, conf, c, output);
    newline(c, output);
    section_body_to_ansi(section, conf, c, output);
}

pub fn heading_to_ansi(heading: &Heading, conf: &Config, c: &mut Context, output: &mut String) {
    c.push_fg_mod(BOLD, output);
    for item in &heading.items {
        match item {
            HeadingItem::String(string) => text_to_ansi(string, c, output),
            HeadingItem::Em(emphasis) => emphasis_to_ansi(emphasis, conf, c, output),
        }
    }
    c.pop_fg_mod(output);
}

pub fn section_body_to_ansi(
    section: &Section, conf: &Config, c: &mut Context, output: &mut String
) {
    for item in &section.items {
        match item {
            SectionItem::Paragraph(par) => {
                c.ps = ParStatus::New;
                c.push_indent(2, 0);
                paragraph_to_ansi(par, conf, c, output);
                c.pop_indent();
            },
            SectionItem::Section(section) => {
                c.push_indent(2, 0);
                section_to_ansi(section, conf, c, output);
                c.pop_indent();
            },
        }
        newline(c, output);
    }
    output.pop();
}

pub fn blockquote_to_ansi(section: &Section, conf: &Config, c: &mut Context, output: &mut String) {
    let mut table = term_table::Table::builder()
        .style(TableStyle::thin())
        .build();
    let mut row = Row::empty();
    let mut temp = String::new();
    if section.tags.contains("blockquote-typed") {
        heading_to_ansi(&section.heading, conf, c, &mut temp);
        newline(c, &mut temp);
    }
    section_body_to_ansi(section, conf, c, &mut temp);
    row.add_cell(TableCell::new(temp));
    table.add_row(row);
    let raw_table = table.render();

    newlines_unless(1, &[ParStatus::New], c, output);
    *output += RESET;
    indent_table(&raw_table, c, output);
    *output += &c.fg_mod;
    c.ps = ParStatus::Element;
}

pub fn paragraph_to_ansi(par: &Paragraph, conf: &Config, c: &mut Context, output: &mut String) {
    for item in &par.items {
        match item {
            ParagraphItem::Text(text) => {
                text_to_ansi(text, c, output);
            },
            ParagraphItem::MText(TextWithMeta { text, tags, .. }) => {
                if tags.contains("code") {
                    inline_code_to_ansi(text, conf, c, output);
                } else {
                    text_to_ansi(text, c, output);
                }
            },
            ParagraphItem::Em(emphasis) => {
                emphasis_to_ansi(emphasis, conf, c, output);
            },
            ParagraphItem::Link(link) => {
                link_to_ansi(link, conf, c, output);
            },
            ParagraphItem::Code(code) => {
                code_to_ansi(code, conf, c, output);
            },
            ParagraphItem::List(list) => {
                list_to_ansi(list, conf, c, output);
            },
            ParagraphItem::Table(table) => {
                table_to_ansi(table, conf, c, output);
            },
        }
    }
}

pub fn list_to_ansi(list: &List, conf: &Config, c: &mut Context, output: &mut String) {
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
        newlines_unless(1, &[], c, output);
        indent(0, c, output);
        match list.ltype {
            ListType::Distinct => append(&format!("{count:>width$}. "), c, output),
            ListType::Identical => append("- ", c, output),
            ListType::Checked if par.tags.contains("checked") => append("- [x] ", c, output),
            ListType::Checked => append("- [ ] ", c, output),
        }
        c.ps = ParStatus::New;
        c.push_indent(iwidth, iwidth);
        paragraph_to_ansi(par, conf, c, output);
        c.pop_indent();
        c.ps = ParStatus::Element;
    }
}

pub fn table_to_ansi(table: &incodoc::Table, conf: &Config, c: &mut Context, output: &mut String) {
    let mut max_cols = 0;
    for row in &table.rows {
        max_cols = max_cols.max(row.items.len());
    }
    let available_width = c.width - c.indentation - max_cols - 1;
    let col_width = available_width / max_cols - 2; // the -2 is for the padding (which is optional)
    let mut t = term_table::Table::builder()
        .style(TableStyle::thin())
        .build();
    for row in &table.rows {
        let mut r = Row::empty();
        for item in &row.items {
            let mut temp = String::new();
            let mut table_context = Context {
                width: col_width,
                ..Default::default()
            };
            paragraph_to_ansi(item, conf, &mut table_context, &mut temp);
            r.add_cell(TableCell::new(temp));
        }
        t.add_row(r);
    }
    let raw_table = t.render();

    newlines_unless(1, &[ParStatus::New], c, output);
    *output += RESET;
    indent_table(&raw_table, c, output);
    *output += &c.fg_mod;
    c.ps = ParStatus::Element;
}

pub fn indent_table(raw_table: &str, c: &mut Context, output: &mut String) {
    let mut indent_string_0 = String::new();
    indent_string_0 += "\n";
    indent(0, c, &mut indent_string_0);
    let mut indent_string_1 = String::new();
    indent_string_1 += "\n";
    indent(0, c, &mut indent_string_1);
    let mut res = String::new();
    res += &indent_string_0[1..];
    res += raw_table;
    res = res.replace("\n", &indent_string_1);
    for _ in 0..indent_string_1.len() {
        res.pop();
    }
    *output += &res;
}

pub fn code_to_ansi(
    code: &Result<CodeBlock, CodeIdentError>,
    conf: &Config,
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
                .term_width(c.width - c.indentation - 2)
                .line_numbers(true)
                .use_italics(true)
                .wrapping_mode(WrappingMode::Character)
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

    newlines_unless(1, &[ParStatus::New], c, output);
    *output += RESET;
    *output += &temp;
    *output += &c.fg_mod;
    c.ps = ParStatus::Element;
}

pub fn inline_code_to_ansi(text: &str, conf: &Config, c: &mut Context, output: &mut String) {
    format_text_pre(c, output);
    *output += RESET;
    c.push_bg_mod(BG_BLACK, output);
    format_text_main(text, c, output);
    c.pop_bg_mod(output);
    c.ps = ParStatus::Char;
}

pub fn link_to_ansi(link: &Link, conf: &Config, c: &mut Context, output: &mut String) {
    c.push_fg_mod(MAGENTA, output);
    for item in &link.items {
        match item {
            LinkItem::String(text) => text_to_ansi(text, c, output),
            LinkItem::Em(em) => emphasis_to_ansi(em, conf, c, output),
        }
    }
    c.pop_fg_mod(output);
}

pub fn emphasis_to_ansi(em: &Emphasis, conf: &Config, c: &mut Context, output: &mut String) {
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
    *output += &c.fg_mod;
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
    if c.col >= c.width {
        newline(c, output);
        return;
    }
    match c.ps {
        ParStatus::Char => {
            output.push(' ');
            c.ps = ParStatus::Whitespace;
            c.col += 1;
        },
        ParStatus::Element => {
            newline(c, output);
        },
        _ => { },
    }
}

pub fn format_text_main(text: &str, c: &mut Context, output: &mut String) {
    for ch in text.chars() {
        if c.col >= c.width {
            newline(c, output);
        }
        if matches!(c.ps, ParStatus::Newline(_)) || c.ps == ParStatus::New {
            indent(0, c, output);
        }
        match ch {
            '\n' => {
                if c.ps == ParStatus::Whitespace || c.ps == ParStatus::Char {
                    c.ps = ParStatus::Newline(1);
                    c.col = 0;
                    output.push('\n');
                }
            },
            '\r' => {},
            x => {
                if x.is_whitespace() {
                    if c.ps != ParStatus::Whitespace {
                        if !matches!(c.ps, ParStatus::Newline(_)) {
                            output.push(' ');
                            c.col += 1;
                        }
                        c.ps = ParStatus::Whitespace;
                    }
                } else {
                    c.ps = ParStatus::Char;
                    c.col += 1;
                    output.push(x);
                }
            },
        }
    }
}

pub fn append(text: &str, c: &mut Context, output: &mut String) {
    let len = text.len();
    if len + c.col < c.width {
        *output += text;
        c.col += len;
    } else {
        let first = len + c.col - c.width;
        *output += &text[..first];
        newline(c, output);
        append(&text[first..], c, output);
    }
}

pub fn indent(extra: usize, c: &mut Context, output: &mut String) {
    *output += RESET;
    let indentation = c.indentation + extra;
    for _ in 0..(indentation - c.indented.min(indentation)) {
        *output += " ";
        c.col += 1;
    }
    c.indented = 0;
    *output += &c.fg_mod;
    *output += &c.bg_mod;
}

pub fn newline(c: &mut Context, output: &mut String) {
    *output += "\n";
    let already = match c.ps {
        ParStatus::Newline(n) => n,
        _ => 0,
    };
    c.ps = ParStatus::Newline(already + 1);
    c.col = 0;
}

pub fn newlines_unless(newlines: usize, pss: &[ParStatus], c: &mut Context, output: &mut String) {
    for ps in pss {
        if c.ps == *ps {
            return;
        }
    }
    let already = match c.ps {
        ParStatus::Newline(n) => n,
        _ => 0,
    };
    let todo = newlines - already.min(newlines);
    for _ in 0..todo {
        *output += "\n";
    }
    c.ps = ParStatus::Newline(already + todo);
    c.col = 0;
}
