use logos::{Logos, Source, Span};

pub struct PeekableLexer<'source, L, T>
where
    T: Logos<'source>,
    L: LogosIter<'source, T>,
{
    lexer: L,
    phantom: std::marker::PhantomData<dyn LogosIter<'source, T>>,
}

pub trait LogosIter<'source, T>: Iterator<Item = T>
where
    T: Logos<'source>,
{
    fn span(&self) -> Span;

    fn slice(&self) -> &'source <T::Source as Source>::Slice<'source>;

    fn source(&self) -> &'source T::Source;

    fn remainder(&self) -> &'source <T::Source as Source>::Slice<'source>;

    fn bump(&mut self, n: usize);

    fn extras(&self) -> &T::Extras;

    fn extras_mut(&mut self) -> &mut T::Extras;

    /// See [`PeekableLexer`].
    // we don't use `peekable` name to avoid ambiguity (it's a compiler error)
    // between `LogosIter`'s method and `Iterator`'s one
    fn peekable_lexer(self) -> PeekableLexer<'source, Self, T>
    where
        Self: Sized,
    {
        PeekableLexer {
            lexer: self,
            phantom: std::marker::PhantomData,
        }
    }
}
