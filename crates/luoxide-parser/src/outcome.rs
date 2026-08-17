#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Outcome<T, E> {
    Ok(T),
    PartialFailure(T, E),
    TotalFailure(E),
}

impl<T, E> Outcome<T, E> {
    /// Maps a `Outcome<T, E>` to `Outcome<U, E>` by applying a function to a
    /// contained [`Outcome::Ok`] or [`Outcome::PartialFailure`] value, leaving an [`Outcome::TotalFailure`] value untouched.
    ///
    pub fn map<U, O: FnOnce(T) -> U>(self, op: O) -> Outcome<U, E> {
        match self {
            Outcome::Ok(t) => Outcome::Ok(op(t)),
            Outcome::PartialFailure(t, e) => Outcome::PartialFailure(op(t), e),
            Outcome::TotalFailure(e) => Outcome::TotalFailure(e),
        }
    }

    /// Maps a `Outcome<T, E>` to `Outcome<U, E>` by applying a function to a
    /// contained [`Outcome::TotalFailure`] or [`Outcome::PartialFailure`] value, leaving an [`Outcome::Ok`] value untouched.
    ///
    pub fn map_err<F, O: FnOnce(E) -> F>(self, op: O) -> Outcome<T, F> {
        match self {
            Outcome::Ok(t) => Outcome::Ok(t),
            Outcome::PartialFailure(t, e) => Outcome::PartialFailure(t, op(e)),
            Outcome::TotalFailure(e) => Outcome::TotalFailure(op(e)),
        }
    }

    pub fn inspect<F: FnOnce(&T)>(self, f: F) -> Self {
        if let Outcome::Ok(ref t) = self {
            f(t);
        }

        self
    }

    pub fn inspect_err<F: FnOnce(&E)>(self, f: F) -> Self {
        if let Outcome::TotalFailure(ref e) = self {
            f(e);
        }

        self
    }

    #[inline]
    pub fn expect(self, msg: &str) -> T
    where
        E: core::fmt::Debug,
    {
        match self {
            Outcome::Ok(t) => t,
            Outcome::PartialFailure(_, e) | Outcome::TotalFailure(e) => unwrap_failed(msg, &e),
        }
    }

    #[inline]
    pub fn expect_err(self, msg: &str) -> E
    where
        T: core::fmt::Debug,
    {
        match self {
            Outcome::PartialFailure(t, _) | Outcome::Ok(t) => unwrap_failed(msg, &t),
            Outcome::TotalFailure(e) => e,
        }
    }

    #[inline]
    pub fn expect_partial(self, msg: &str) -> (T, E)
    where
        E: core::fmt::Debug,
        T: core::fmt::Debug,
    {
        match self {
            Outcome::Ok(t) => unwrap_failed(msg, &t),
            Outcome::PartialFailure(t, e) => (t, e),
            Outcome::TotalFailure(e) => unwrap_failed(msg, &e),
        }
    }

    #[inline(always)]
    pub fn unwrap(self) -> T
    where
        E: core::fmt::Debug,
    {
        match self {
            Outcome::Ok(t) => t,
            Outcome::PartialFailure(_, e) => unwrap_failed(
                "called `Outcome::unwrap()` on an `PartialFailure` value",
                &e,
            ),
            Outcome::TotalFailure(e) => {
                unwrap_failed("called `Outcome::unwrap()` on an `TotalFailure` value", &e)
            }
        }
    }

    #[inline]
    pub fn unwrap_err(self) -> E
    where
        T: core::fmt::Debug,
    {
        match self {
            Outcome::Ok(t) => unwrap_failed("called `Outcome::unwrap_err()` on an `Ok` value", &t),
            Outcome::PartialFailure(t, _) => unwrap_failed(
                "called `Outcome::unwrap_err()` on an `PartialFailure` value",
                &t,
            ),
            Outcome::TotalFailure(e) => e,
        }
    }

    #[inline]
    pub fn unwrap_partial(self) -> (T, E)
    where
        E: core::fmt::Debug,
        T: core::fmt::Debug,
    {
        match self {
            Outcome::Ok(t) => {
                unwrap_failed("called `Outcome::unwrap_partial()` on an `Ok` value", &t)
            }
            Outcome::PartialFailure(t, e) => (t, e),
            Outcome::TotalFailure(e) => unwrap_failed(
                "called `Outcome::unwrap_partial()` on an `TotalFailure` value",
                &e,
            ),
        }
    }

    #[inline]
    pub fn into_result(self) -> Result<T, (Option<T>, E)> {
        self.into()
    }
}

impl<T, E> Outcome<&T, E> {
    #[inline]
    pub fn copied(self) -> Outcome<T, E>
    where
        T: Copy,
    {
        self.map(|&t| t)
    }

    #[inline]
    pub fn cloned(self) -> Outcome<T, E>
    where
        T: Clone,
    {
        self.map(|t| t.clone())
    }
}

impl<T, E> Outcome<&mut T, E> {
    #[inline]
    pub fn copied(self) -> Outcome<T, E>
    where
        T: Copy,
    {
        self.map(|&mut t| t)
    }

    #[inline]
    pub fn cloned(self) -> Outcome<T, E>
    where
        T: Clone,
    {
        self.map(|t| t.clone())
    }
}

impl<T, E> Clone for Outcome<T, E>
where
    T: Clone,
    E: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        match self {
            Outcome::Ok(x) => Outcome::Ok(x.clone()),
            Outcome::PartialFailure(t, e) => Outcome::PartialFailure(t.clone(), e.clone()),
            Outcome::TotalFailure(x) => Outcome::TotalFailure(x.clone()),
        }
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        match (self, source) {
            (Outcome::Ok(to), Outcome::Ok(from)) => to.clone_from(from),
            (Outcome::PartialFailure(to_t, to_e), Outcome::PartialFailure(from_t, from_e)) => {
                to_t.clone_from(from_t);
                to_e.clone_from(from_e)
            }
            (Outcome::TotalFailure(to), Outcome::TotalFailure(from)) => to.clone_from(from),
            (to, from) => *to = from.clone(),
        }
    }
}

impl<T, E> Into<Result<T, (Option<T>, E)>> for Outcome<T, E> {
    fn into(self) -> Result<T, (Option<T>, E)> {
        match self {
            Outcome::Ok(t) => Ok(t),
            Outcome::PartialFailure(t, e) => Err((Some(t), e)),
            Outcome::TotalFailure(e) => Err((None, e)),
        }
    }
}

#[inline(never)]
#[cold]
#[track_caller]
fn unwrap_failed(msg: &str, error: &dyn core::fmt::Debug) -> ! {
    panic!("{msg}: {error:?}")
}
