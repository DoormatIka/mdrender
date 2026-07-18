use mdrenderlib::parser::parser;
use std::fs;

fn main() {
    let contents = fs::read_to_string("tests/bless.txt").unwrap();

    let mut parsee = parser::BlockParser::new();
    parsee.parse_phase_one(contents);

    println!("{}", parsee);
}
