use crate::terminal::Terminal;

pub trait Highlighter {
    fn highlight(&self, content: &str, terminal: &Terminal);
}

