use crate::block::parser::BlockParser;

impl BlockParser {
    pub(in crate::block) fn match_block_quote(line: &str) -> Option<&str> {
        let trimmed = BlockParser::check_indent(line)?;

        trimmed
            .strip_prefix('>')
            .map(|r| r.strip_prefix(' ').unwrap_or(r))
    }
}
