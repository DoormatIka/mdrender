use mdrenderlib::parser::parser;

fn main() {
    let text = "
        Line one, Line one.

        Line two, line two.
        ";

    let parsee = parser::CommonmarkParser::new();
    parsee.parse(text.to_string());
}
