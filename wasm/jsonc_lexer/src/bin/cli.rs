use std::env;
use std::io::{self, Read};

use jsonc_lexer::jsonc_lexer_native;

fn main() {
    let mut args = env::args().skip(1);
    if let Some(inline_input) = args.next() {
        // Use Debug format since TokenizeResult doesn't implement Display
        println!("{:#?}", jsonc_lexer_native(&inline_input));
        return;
    }

    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("failed to read from stdin");
    // Use Debug format since TokenizeResult doesn't implement Display
    print!("{:#?}", jsonc_lexer_native(&buffer));
}
