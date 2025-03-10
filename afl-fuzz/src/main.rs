extern crate afl;
extern crate xmlparser_relaxed;

use std::str;

use afl::fuzz;

fn main() {
    fuzz!(|data: &[u8]| {
        if let Ok(text) = str::from_utf8(data) {
            for _ in xmlparser_relaxed::Tokenizer::from(text) {}
        }
    });
}
