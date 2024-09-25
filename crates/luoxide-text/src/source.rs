use crate::{range::TextSpan, size::TextSize};

const fn is_newline(c: char) -> bool {
    matches!(c, '\r' | '\n')
}

pub struct Source<'src> {
    inner: &'src str,
}

impl<'src> Source<'src> {
    pub fn new(source: &'src str) -> Self {
        Self { inner: source }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn try_text_len(&self) -> Result<usize, ()> {
        if self.len() > u32::MAX as usize {
            return Err(());
        }
        Ok(self.len())
    }

    pub fn text(&self) -> &'src str {
        self.inner
    }

    pub fn text_len(&self) -> TextSize {
        TextSize::new(self.len() as u32)
    }

    pub fn lexeme(&self, range: TextSpan) -> &'src str {
        &self.inner[range]
    }

    pub fn find_last_newline(&self, location: TextSize) -> Option<usize> {
        if self.inner.is_empty() {
            return None;
        }
        self.inner[..location.to_usize()]
            .rfind(is_newline)
            .map(|i| i + 1)
    }

    /*pub fn find_line_number(&self, location: TextSize) -> usize {
        self.inner[..location.to_usize()].matches(is_newline).count() + 1
    }*/

    pub fn get_all_line_indexes(&self) -> Vec<usize> {
        self.inner
            .split(is_newline)
            .filter(|s| !s.is_empty())
            .enumerate()
            .map(|(i, _)| i + 1)
            .collect()
    }
}
