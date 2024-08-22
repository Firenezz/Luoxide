use crate::token::TokenKind;

type TokenKindSet = u128;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct TokenSet(TokenKindSet);

impl TokenSet {
    pub(crate) const EMPTY: TokenSet = TokenSet(0);

    pub(crate) const fn new<const N: usize>(tokenkinds: [TokenKind; N]) -> TokenSet {
        let mut set = Self::EMPTY;
        let mut i = 0;
        while i < N {
            let token_kind = tokenkinds[i];
            set.0 |= mask(token_kind);
            i += 1;
        }

        set
    }

    #[inline]
    pub(crate) const fn union(self, other: TokenSet) -> TokenSet {
        TokenSet(self.0 | other.0)
    }

    #[inline]
    pub(crate) const fn remove(&self, kind: TokenKind) -> TokenSet {
        TokenSet(self.0 & !mask(kind))
    }

    #[inline]
    pub(crate) const fn contains_set(&self, token: TokenKindSet) -> bool {
        self.0 & token != 0
    }

    #[inline]
    pub(crate) const fn contains(&self, kind: TokenKind) -> bool {
        self.contains_set(mask(kind))
    }
}

#[inline]
const fn mask(kind: TokenKind) -> TokenKindSet {
    (1 as TokenKindSet) << (kind as TokenKindSet)
}

impl<const N: usize> From<[TokenKind; N]> for TokenSet {
    fn from(kinds: [TokenKind; N]) -> Self {
        TokenSet::new(kinds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_works() {
        let mut set = TokenSet::new([token!(do), token!(if)]);

        assert!(set.contains(token!(do)));
        assert!(set.contains(token!(if)));

        assert!(set.contains_set(mask(token!(do)) | mask(token!(if))));

        assert!(!set.contains(token!(else)));

        set = set.remove(token!(do));

        assert!(!set.contains_set(mask(token!(do))));
        assert!(set.contains_set(mask(token!(if))));

        let set2: TokenSet = [token!(function), token!(end), token!(EOF)].into();

        set = set.union(set2);

        assert!(set.contains_set(mask(token!(function)) | mask(token!(end))));

        assert!(!set.contains_set(mask(token!(do))));
        assert!(set.contains_set(mask(token!(if))));

        assert!(!set.contains_set(mask(token!(else))));
    }
}
