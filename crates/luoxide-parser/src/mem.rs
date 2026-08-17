#[allow(dead_code)]
pub struct InternId(u32);

impl InternId {
    pub fn new() -> Self {
        Self(0)
    }
}

impl Default for InternId {
    fn default() -> Self {
        Self::new()
    }
}
