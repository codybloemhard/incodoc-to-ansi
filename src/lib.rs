use incodoc::*;

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Context {
    pub ps: ParStatus,
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

pub fn doc_to_ansi_string(doc: &Doc) -> String {
    let mut res = String::new();
    let mut context = Context::default();
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
                print!("{}", format_text(text, c));
            },
            ParagraphItem::MText(TextWithMeta { text, tags, props }) => {
                print!("{}", format_text(text, c));
            },
            ParagraphItem::Em(emphasis) => {},
            ParagraphItem::Link(link) => {},
            ParagraphItem::Code(code) => {},
            ParagraphItem::List(list) => {},
            ParagraphItem::Table(table) => {},
        }
    }
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
