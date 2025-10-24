
#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Config {
    pub width: usize,
    pub nav: NavConfig,
    pub section: SectionConfig,
    pub headed_section: HeadedSectionConfig,
    pub code_block: CodeBlockConfig,
    pub blockquote: BlockquoteConfig,
    pub list: ListConfig,
    pub table: TableConfig,
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

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct BlockquoteConfig {
    pub pre_quote_mns: usize,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct CodeBlockConfig {
    pub indent: usize,
    pub pre_code_block_mns: usize,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ListConfig {
    pub pre_item_mns: usize,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct TableConfig {
    pub pre_table_mns: usize,
}
