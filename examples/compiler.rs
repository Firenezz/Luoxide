fn main() {
    let source = r##"
"test".."\n".. " \x0F" .."test2"
"##;
    dbg!(luoxidant::public::parse_expression(source).unwrap());
}
