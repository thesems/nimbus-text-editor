#[derive(PartialEq, Eq, Hash)]
pub enum FileExtension {
    Rust,
    Toml,
}

impl FileExtension {
    fn as_str(&self) -> &'static str {
        match self {
            FileExtension::Rust => "rs",
            FileExtension::Toml => "toml",
        }
    }
}
