#[derive(Clone, PartialEq, Eq, Hash)]
pub enum FileExtension {
    Rust,
    Toml,
    Text,
    Unknown,
}

impl FileExtension {
    pub fn as_str(&self) -> &'static str {
        match self {
            FileExtension::Rust => "rs",
            FileExtension::Toml => "toml",
            FileExtension::Text => "text",
            FileExtension::Unknown => "unknown",
        }
    }
}
