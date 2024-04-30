fn main() {
    let source = r##"
"\x30\u{1D306}\064".."\u{4000}"
"##;
    dbg!(luoxidant::public::parse_expression(source).unwrap());
}
