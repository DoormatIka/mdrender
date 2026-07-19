use mdrenderlib::block::parser;
use std::fs;

fn main() {
    // let a = parser::BlockParser::match_heading("### text ###");
    parse_text_file();
}

fn parse_text_file() {
    let contents = fs::read_to_string("tests/bless.txt").unwrap();

    let mut parsee = parser::BlockParser::new();
    parsee.parse_phase_one(contents);

    println!("{}", parsee);
}
