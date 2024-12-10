use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use std::hint::black_box;

use luoxide_parser::{token::TokenKind, token_set::TokenSet};
use rand::{seq::SliceRandom, thread_rng};

const TOKEN_SET: TokenSet = TokenSet::new([
    luoxide_parser::token::TokenKind::Nil,
    luoxide_parser::token::TokenKind::Lit_True,
    luoxide_parser::token::TokenKind::Lit_False,
    luoxide_parser::token::TokenKind::Lit_Number,
    luoxide_parser::token::TokenKind::Lit_HexNumber,
    luoxide_parser::token::TokenKind::Lit_Float,
    luoxide_parser::token::TokenKind::Lit_HexFloat,
    luoxide_parser::token::TokenKind::Lit_Identifier,
    luoxide_parser::token::TokenKind::Lit_String,
    luoxide_parser::token::TokenKind::Lit_MultilineString,
    luoxide_parser::token::TokenKind::NaN,
]);

fn token_set_match(kind: TokenKind) -> bool {
    TOKEN_SET.contains(kind)
}

fn token_set(c: &mut Criterion) {
    c.bench_function("token set", |b| {
        b.iter(|| token_set_match(black_box(TokenKind::Lit_Identifier)))
    });
}

fn matches_match(kind: TokenKind) -> bool {
    matches!(
        kind,
        luoxide_parser::token::TokenKind::Nil
            | luoxide_parser::token::TokenKind::Lit_True
            | luoxide_parser::token::TokenKind::Lit_False
            | luoxide_parser::token::TokenKind::Lit_Number
            | luoxide_parser::token::TokenKind::Lit_HexNumber
            | luoxide_parser::token::TokenKind::Lit_Float
            | luoxide_parser::token::TokenKind::Lit_HexFloat
            | luoxide_parser::token::TokenKind::Lit_Identifier
            | luoxide_parser::token::TokenKind::Lit_String
            | luoxide_parser::token::TokenKind::Lit_MultilineString
            | luoxide_parser::token::TokenKind::NaN
    )
}

fn matches(c: &mut Criterion) {
    c.bench_function("token match", |b| {
        b.iter(|| matches_match(black_box(TokenKind::Lit_Identifier)))
    });
}

criterion_group!(
    benches,
    token_set,
    matches,
    token_set_random,
    matches_random
);

criterion_main!(benches);

fn token_set_random(c: &mut Criterion) {
    c.bench_function("token set random", |b| {
        b.iter_batched(
            || black_box(random_token_kind()),
            token_set_match,
            BatchSize::SmallInput,
        )
    });
}

fn matches_random(c: &mut Criterion) {
    c.bench_function("token match random", |b| {
        b.iter_batched(
            || black_box(random_token_kind()),
            matches_match,
            BatchSize::SmallInput,
        )
    });
}

fn random_token_kind() -> TokenKind {
    static TOKEN_KINDS: &[TokenKind] = &[
        TokenKind::Nil,
        TokenKind::Lit_True,
        TokenKind::Lit_False,
        TokenKind::Lit_Number,
        TokenKind::Lit_HexNumber,
        TokenKind::Lit_Float,
        TokenKind::Lit_HexFloat,
        TokenKind::Lit_Identifier,
        TokenKind::Lit_String,
        TokenKind::Lit_MultilineString,
        TokenKind::NaN,
    ];
    *TOKEN_KINDS.choose(&mut thread_rng()).unwrap()
}
