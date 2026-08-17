pub mod intern;

// Reuse Rustc ast pointer type
pub struct P<T> {
    ptr: Box<T>,
}

impl<T> P<T> {
    pub fn new(ptr: T) -> Self {
        Self { ptr: Box::new(ptr) }
    }
}
