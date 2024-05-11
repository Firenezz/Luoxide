fn main() {
    /*let source = r##"
    "\x30\u{1D306}\064".."\u{4000}"
    "##;
        dbg!(luoxidant::public::parse_expression(source).unwrap());*/
    let source = r##"
    x, y ="test", 1 * 2 + 3 * (4 + 5 * 6)
    "##;
    dbg!(luoxidant::public::parse_chunk(source).unwrap());
    println!("{}", source);
}
