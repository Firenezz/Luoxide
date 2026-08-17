use luoxide::prelude::*;

fn main() {
    let mut session = Session::new();
    let chunk = session.parse_chunk("print(1)").unwrap();
    println!("{}", session.display(&chunk, "print(1)"));
}
