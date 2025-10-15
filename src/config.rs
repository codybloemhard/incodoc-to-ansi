
#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Config {
    pub width: usize,
    pub nav_config: NavConfig,
    pub section_config: SectionConfig,
    pub code_block_config: CodeBlockConfig,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct NavConfig {
    pub link_indent: usize,
    pub sub_indent: usize,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct SectionConfig {
    pub paragraph_indent: usize,
    pub section_indent: usize,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct CodeBlockConfig {
    pub indent: usize,
}
