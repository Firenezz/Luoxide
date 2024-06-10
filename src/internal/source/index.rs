use super::text::TextSize;

pub struct LineIndex {
    pub line: TextSize,
}

impl LineIndex {
    pub fn new(index: TextSize) -> LineIndex {
        LineIndex { line: index }
    }

    pub fn line(&self) -> TextSize {
        self.line
    }
}
