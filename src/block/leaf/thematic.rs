use crate::block::parser::BlockParser;

impl BlockParser {
    pub(in crate::block) fn match_thematic_break(line: &str) -> Option<&str> {
        let trimmed = Self::check_indent(line)?;

        let mut breakline = 0;

        let mut previous_char = ' ';
        for ch in trimmed.chars() {
            let is_symbol = matches!(ch, '*' | '-' | '_' | ' ' | '\t');
            let is_char_allowed = is_symbol || previous_char == ch;

            if !is_char_allowed {
                breakline = 0;
            }

            if matches!(ch, '*' | '-' | '_' | ' ' | '\t') {
                breakline += 1;
            } else {
                // for other characters, this isn't a thematic break.
                return None;
            }

            previous_char = ch;
        }

        if breakline > 2 {
            Some(&line[line.len()..]) // assuming there's nothing left after parsing this.
        } else {
            None
        }
    }
}
