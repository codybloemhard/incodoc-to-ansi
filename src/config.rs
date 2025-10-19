
#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Config {
    pub width: usize,
    pub nav_config: NavConfig,
    pub section_config: SectionConfig,
    pub headed_section_config: HeadedSectionConfig,
    pub code_block_config: CodeBlockConfig,
    pub blockquote_config: BlockquoteConfig,
    pub list_config: ListConfig,
    pub table_config: TableConfig,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct NavConfig {
    pub link_indent: usize,
    pub sub_indent: usize,
    pub pre_description_newlines: usize,
    pub post_description_newlines: usize,
    pub pre_link_newlines: usize,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct SectionConfig {
    pub paragraph_indent: usize,
    pub section_indent: usize,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct HeadedSectionConfig {
    pub pre_heading_newlines: usize,
    pub post_heading_newlines: usize,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct BlockquoteConfig {
    pub pre_quote_newlines: usize,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct CodeBlockConfig {
    pub indent: usize,
    pub pre_code_block_newlines: usize,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ListConfig {
    pub pre_item_newlines: usize,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct TableConfig {
    pub pre_table_newlines: usize,
}
