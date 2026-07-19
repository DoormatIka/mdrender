use crate::block::parser::BlockParser;

impl BlockParser {
    pub(in crate::block) fn match_paragraph(line: &str) -> Option<&str> {
        if line.trim().is_empty() {
            return None;
        }
        if Self::match_block_quote(line).is_some()
            || Self::match_thematic_break(line).is_some()
            || Self::match_atx_heading(line).is_some()
        {
            return None;
        }
        Some(line)
    }
}
