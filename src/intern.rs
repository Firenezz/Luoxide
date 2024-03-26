use core::cell::RefCell;

use std::collections::HashSet;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use crate::span::Span;

#[derive(Default, Debug)]
pub struct AsyncDefaultInterner(RwLock<HashSet<Arc<[u8]>>>);
#[derive(Default, Debug)]
pub struct DefaultInterner(RefCell<HashSet<Rc<[u8]>>>);

pub trait StringInterner {
    type String: AsRef<[u8]>;

    fn intern(&self, s: impl AsRef<[u8]>) -> Self::String;
}

impl<'a, S: StringInterner> StringInterner for &'a S {
    type String = S::String;

    fn intern(&self, s: impl AsRef<[u8]>) -> Self::String {
        S::intern(self, s)
    }
}

impl StringInterner for DefaultInterner {
    type String = Rc<[u8]>;

    fn intern(&self, string: impl AsRef<[u8]>) -> Self::String {
        let mut set = self.0.borrow_mut();
        if let Some(s) = set.get(string.as_ref()) {
            s.clone()
        } else {
            let s = Rc::from(string.as_ref().to_owned());
            set.insert(Rc::clone(&s));
            s
        }
    }
}

impl StringInterner for AsyncDefaultInterner {
    type String = Arc<[u8]>;

    fn intern(&self, string: impl AsRef<[u8]>) -> Self::String {
        let set = self.0.read().unwrap();
        if let Some(s) = set.get(string.as_ref()) {
            s.clone()
        } else {
            drop(set);
            let mut set = self.0.write().unwrap();
            let s = Arc::from(string.as_ref().to_owned());
            set.insert(Arc::clone(&s));
            s
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_interner() {
        // Arrange
        let interner = DefaultInterner::default();

        // Act
        let interned_string = interner.intern(b"hello");
        let interned_string2 = interner.intern(b"world");

        // Assert

        // Assert that the strings are equal
        assert_eq!(interned_string.as_ref(), b"hello");
        assert_eq!(interned_string2.as_ref(), b"world");
        assert_eq!(interner.intern(b"hello"), interned_string); // Should return the same instance as the first interned string

        // Assert amount of strings
        assert_eq!(interner.0.borrow().len(), 2);
    }

    #[test]
    fn test_async_default_interner() {
        // Arrange
        let interner = AsyncDefaultInterner::default();

        // Act
        let interned_string = interner.intern(b"hello");
        let interned_string2 = interner.intern(b"world");

        // Assert

        // Assert that the strings are equal
        assert_eq!(interned_string.as_ref(), b"hello");
        assert_eq!(interned_string2.as_ref(), b"world");
        assert_eq!(interner.intern(b"hello"), interned_string); // Should return the same instance as the first interned string

        // Assert amount of strings
        assert_eq!(interner.0.read().unwrap().len(), 2);
    }

    #[test]
    fn test_concurrent_async_default_interner() {
        use std::borrow::BorrowMut;
        use std::sync::Barrier;
        use std::thread;
        // Arrange
        let interner = Arc::new(AsyncDefaultInterner::default());
        const THREADS: usize = 10;
        let barrier = Arc::new(Barrier::new(THREADS));

        let mut handles = vec![];

        // Act
        for _ in 0..THREADS {
            let mut interner = Arc::clone(&interner);
            let barrier = Arc::clone(&barrier);
            let handle = thread::spawn(move || {
                barrier.wait(); // Wait for all threads to reach this point
                interner.borrow_mut().intern(b"hello")
            });
            handles.push(handle);
        }

        // Assert

        for handle in handles {
            let handle = handle.join().unwrap();
            // Assert that the string is equal
            assert_eq!(handle, Arc::from(&b"hello"[..]));
        }

        // Assert amount of strings
        assert_eq!(interner.0.read().unwrap().len(), 1);
    }

    #[test]
    fn test_stress_concurrent_async_default_interner() {
        use rand::distributions::{Distribution, Uniform};
        use std::{iter, sync::Barrier, thread};

        // Arrange
        const CHARACTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()_-+=[]{}\\|;:'\",.<>/?`~";

        let interner = Arc::new(AsyncDefaultInterner::default());
        const THREADS: usize = 16;
        const ITERATIONS: usize = 10000;
        let barrier = Arc::new(Barrier::new(THREADS));
        let mut strings = vec![b"hello".to_owned(), b"world".to_owned()];
        let iterator = iter::repeat_with(|| {
            let mut rng = rand::thread_rng();
            let range = Uniform::new(0, CHARACTERS.len());
            let indices: Vec<usize> = iter::repeat(())
                .map(|()| range.sample(&mut rng))
                .take(5)
                .collect();
            let mut characters: [u8; 5] = [0; 5];
            for (i, index) in indices.iter().enumerate() {
                characters[i] = CHARACTERS.chars().nth(*index).unwrap() as u8;
            }
            characters
        });
        strings.append(&mut iterator.take(3).collect::<Vec<_>>());

        let strings_len = strings.len();

        let mut handles = vec![];

        // Act
        for _ in 0..THREADS {
            let interner = Arc::clone(&interner);
            let barrier = Arc::clone(&barrier);
            let strings = strings.clone();
            let handle = thread::spawn(move || {
                barrier.wait(); // Wait for all threads to reach this point
                let interner_output = strings
                    .clone()
                    .iter()
                    .map(|string_to_intern| interner.intern(string_to_intern))
                    .collect::<Vec<_>>();
                for i in 0..ITERATIONS {
                    let index = i % strings_len;
                    let chosen_string = strings[index];
                    assert!(interner_output.contains(&interner.intern(chosen_string)));
                }
            });
            handles.push(handle);
        }

        // Assert

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // Assert the number of strings
        assert_eq!(interner.0.read().unwrap().len(), strings.len());
    }
}
