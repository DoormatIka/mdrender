use crate::block::parser::BlockParser;

impl BlockParser {
    pub(in crate::block) fn match_atx_heading(line: &str) -> Option<(u8, &str)> {
        let trimmed = Self::check_indent(line)?;
        let trimmed = trimmed.trim_end();

        // count leading #'s for heading level
        let level = trimmed.chars().take_while(|&c| c == '#').count();
        if level == 0 || level > 6 {
            return None;
        }
        let after_hashes = &trimmed[level..];

        // followed by whitespace, rules out #hashtag
        let text = match after_hashes.strip_prefix([' ', '\t']) {
            Some(rest) => rest.trim(),
            None if after_hashes.is_empty() => "",
            None => return None,
        };

        let text = Self::strip_closing_hashes(text);

        Some((level as u8, text))
    }
    fn strip_closing_hashes(text: &str) -> &str {
        let trimmed_hashes = text.trim_end_matches('#');
        let hash_count = text.len() - trimmed_hashes.len();

        // no trailing '#'s, or none were stripped at all: nothing to do
        if hash_count == 0 {
            return text;
        }

        // closing sequence must be preceded by whitespace (or be the whole string)
        match trimmed_hashes.chars().last() {
            None => trimmed_hashes,
            Some(c) if c.is_whitespace() => trimmed_hashes.trim_end(),
            _ => text, // e.g. "hi#", not a valid closing sequence
        }
    }
}
