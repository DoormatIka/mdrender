use mdrenderlib::parser::parser;

fn main() {
    let text = "Line one, Line one.
> > Line two, line two.
Line three.
";

    let mut parsee = parser::BlockParser::new();
    parsee.parse_phase_one(text.to_string());

    println!("{}", parsee);
}
