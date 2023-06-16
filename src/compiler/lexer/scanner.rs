use crate::intern::StringInterner;



pub struct Scanner<'a, S: StringInterner> {
    source: &'a [u8],
    interner: S,
    start: usize,
    current: usize,
    line: usize,
}