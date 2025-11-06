use zen_colour::*;

#[derive(Clone, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Config {
    pub width: usize,
    pub nav: NavConfig,
    pub section: SectionConfig,
    pub headed_section: HeadedSectionConfig,
    pub heading: HeadingConfig,
    pub code_block: CodeBlockConfig,
    pub code_inline: CodeInlineConfig,
    pub blockquote: BlockquoteConfig,
    pub list: ListConfig,
    pub table: TableConfig,
    pub link: LinkConfig,
    pub text: TextConfig,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct NavConfig {
    pub link_indent: usize,
    pub sub_indent: usize,
    pub pre_description_mns: usize,
    pub post_description_ns: usize,
    pub pre_link_mns: usize,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct SectionConfig {
    pub paragraph_indent: usize,
    pub section_indent: usize,
    pub pre_item_mns: usize,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct HeadedSectionConfig {
    pub pre_heading_mns: usize,
    pub post_heading_ns: usize,
}

#[derive(Clone, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct HeadingConfig {
    pub ansi_mod: AnsiMod,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct BlockquoteConfig {
    pub pre_quote_mns: usize,
}

#[derive(Clone, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct CodeBlockConfig {
    pub indent: usize,
    pub pre_code_block_mns: usize,
    pub bat_theme: String,
    pub show_line_numbers: bool,
    pub use_italics: bool,
}

#[derive(Clone, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct CodeInlineConfig {
    pub ansi_mod: AnsiMod,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ListConfig {
    pub pre_item_mns: usize,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct TableConfig {
    pub pre_table_mns: usize,
}

#[derive(Clone, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct LinkConfig {
    pub ansi_mod: AnsiMod,
}

#[derive(Clone, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct TextConfig {
    pub swallow_whitespace: bool,
    pub whitespace_swallowers: String,
}

#[derive(Clone, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct AnsiMod {
    pub fg: Option<ColMod>,
    pub bg: Option<ColMod>,
    pub fx: Option<Vec<FxMod>>,
}

#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum FxMod {
    Bold,
    Faint,
    Italic,
    Underlined,
    Blink,
    Effect6,
    Effect7,
    Hidden,
    Crossed,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ColMod {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    #[default]
    Default,
}

impl AnsiMod {
    pub fn inherit(mut self, prev: &Self) -> Self {
        if self.fg.is_none() {
            self.fg = prev.fg;
        }
        if self.bg.is_none() {
            self.bg = prev.bg;
        }
        let mut temp = Vec::new();
        if let Some(fxs) = &prev.fx {
            for fx in fxs {
                temp.push(*fx);
            }
        }
        if self.fx.is_none() && !temp.is_empty() {
            self.fx = Some(temp);
        } else if let Some(ref mut fxs) = self.fx {
            for fx in temp {
                fxs.push(fx);
            }
        }
        self
    }

    pub fn to_ansi_string(&self) -> String {
        fn fg_col_to_ansi(c: &ColMod) -> &str {
            match c {
                ColMod::Black => BLACK,
                ColMod::Red => RED,
                ColMod::Green => GREEN,
                ColMod::Yellow => YELLOW,
                ColMod::Blue => BLUE,
                ColMod::Magenta => MAGENTA,
                ColMod::Cyan => CYAN,
                ColMod::White => WHITE,
                ColMod::Default => DEFAULT,
            }
        }
        fn bg_col_to_ansi(c: &ColMod) -> &str {
            match c {
                ColMod::Black => BG_BLACK,
                ColMod::Red => BG_RED,
                ColMod::Green => BG_GREEN,
                ColMod::Yellow => BG_YELLOW,
                ColMod::Blue => BG_BLUE,
                ColMod::Magenta => BG_MAGENTA,
                ColMod::Cyan => BG_CYAN,
                ColMod::White => BG_WHITE,
                ColMod::Default => BG_DEFAULT,
            }
        }
        fn fx_to_ansi(f: &FxMod) -> &str {
            match f {
                FxMod::Bold => BOLD,
                FxMod::Faint => FAINT,
                FxMod::Italic => ITALIC,
                FxMod::Underlined => UNDERLINED,
                FxMod::Blink => BLINK,
                FxMod::Effect6 => EFFECT6,
                FxMod::Effect7 => EFFECT7,
                FxMod::Hidden => HIDDEN,
                FxMod::Crossed => CROSSED,
            }
        }
        let mut res = String::new();
        if let Some(fg) = self.fg {
            res.push_str(fg_col_to_ansi(&fg));
        }
        if let Some(bg) = self.bg {
            res.push_str(bg_col_to_ansi(&bg));
        }
        if let Some(fxs) = &self.fx {
            for fx in fxs {
                res.push_str(fx_to_ansi(fx));
            }
        }
        res
    }
}
