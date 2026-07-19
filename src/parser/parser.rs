use crate::parser::blocks::{Arena, Document, OpenBlock, OpenBlockKind, RawBlock};

pub struct BlockParser {
    pub arena: Arena,
    pub open_stack: Vec<usize>,
}
impl Default for BlockParser {
    fn default() -> Self {
        let mut arena = Arena::new();
        let root = arena.push(OpenBlock::new(OpenBlockKind::Document, None));
        Self {
            arena,
            open_stack: vec![root],
        }
    }
}
impl BlockParser {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn parse_phase_one(&mut self, input: String) -> Document<RawBlock> {
        let doc = Document::<RawBlock>::new();

        let input = input.replace("\u{0000}", "\u{FFFD}");
        for line in input.lines() {
            self.process_line(line);
        }
        self.close_blocks_below(0); // to close the document struct

        doc
    }

    fn print_all_open_stack(&self, label: String) {
        let s = format!("==== {} ====", label);
        let padding = "=".repeat(s.chars().count());
        println!("{}", s);

        for index in self.open_stack.iter() {
            let node = self.arena.get(*index);
            if let Some(node) = node {
                println!("{}", node);
            }
        }
        println!("{}", padding);
    }

    fn process_line(&mut self, line: &str) {
        let (depth, remainder) = self.match_open_blocks(line);

        self.close_blocks_below(depth);
        let remainder = self.try_open_new_blocks(remainder);

        self.append_text(remainder);
    }

    // checking for any open blocks (to catch block types that span for more than one line).
    fn match_open_blocks<'a>(&self, line: &'a str) -> (usize, &'a str) {
        let mut remaining = line;
        for (depth, &idx) in self.open_stack.iter().enumerate() {
            let node = self.arena.get(idx);
            if let Some(node) = node {
                let kind = &node.kind;
                match kind {
                    OpenBlockKind::Document => {
                        continue;
                    }
                    OpenBlockKind::BlockQuote => match Self::match_block_quote(remaining) {
                        Some(r) => remaining = r,
                        None => return (depth, remaining),
                    },
                    OpenBlockKind::Paragraph(_) => match Self::match_paragraph(remaining) {
                        Some(r) => remaining = r,
                        None => return (depth, remaining),
                    },
                    OpenBlockKind::ThematicBreak | OpenBlockKind::Heading { .. } => {
                        continue;
                    }
                    _ => todo!(),
                }
            }
        }

        (self.open_stack.len(), remaining)
    }

    fn match_paragraph(line: &str) -> Option<&str> {
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

    fn check_indent(line: &str) -> Option<&str> {
        let indent = line.len() - line.trim_start_matches(' ').len();
        if indent > 3 {
            return None; // call indented code block instead for 4+ spaces.
        }
        Some(&line[indent..])
    }

    fn match_atx_heading(line: &str) -> Option<(u8, &str)> {
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

    fn match_thematic_break(line: &str) -> Option<&str> {
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

    fn match_block_quote(line: &str) -> Option<&str> {
        let trimmed = Self::check_indent(line)?;

        trimmed
            .strip_prefix('>')
            .map(|r| r.strip_prefix(' ').unwrap_or(r))
    }

    /// closing
    fn close_blocks_below(&mut self, depth: usize) {
        while self.open_stack.len() > depth {
            // maybe >= depth to catch
            let idx = self.open_stack.pop().unwrap();
            self.arena.get_mut(idx).unwrap().is_open = false;
            // not handling lazy continuation yet.
        }
    }

    /// opening/creating new blocks
    fn try_open_new_blocks<'a>(&mut self, remainder: &'a str) -> &'a str {
        let mut rest = remainder;

        loop {
            if let Some(rem) = Self::match_block_quote(rest) {
                let parent = *self.open_stack.last().unwrap();
                let block = OpenBlock::new(OpenBlockKind::BlockQuote, Some(parent));

                let idx = self.arena.push(block);
                self.open_stack.push(idx);
                rest = rem;

                continue;
            }
            if let Some((level, raw)) = Self::match_atx_heading(rest) {
                let parent = *self.open_stack.last().unwrap();
                let mut block = OpenBlock::new(
                    OpenBlockKind::Heading {
                        level,
                        raw: raw.to_string(),
                    },
                    Some(parent),
                );
                block.is_open = false;

                self.arena.push(block);
                rest = "";

                break;
            }
            if Self::match_thematic_break(rest).is_some() {
                let parent = *self.open_stack.last().unwrap();
                let mut block = OpenBlock::new(OpenBlockKind::ThematicBreak, Some(parent));
                block.is_open = false;

                self.arena.push(block);
                rest = "";

                break;
            }

            break;
        }

        rest
    }

    // just appending the existing text as a fallback
    fn append_text(&mut self, remainder: &str) {
        let last = *self.open_stack.last().unwrap();
        let remainder = remainder.to_string();

        if remainder.trim().is_empty() {
            return;
        }

        if let Some(block) = self.arena.get_mut(last) {
            // why does this specific line crash my
            // entire debugger..
            match &mut block.kind {
                OpenBlockKind::Paragraph(p) => {
                    p.push(remainder);
                }
                _ => {
                    let block =
                        OpenBlock::new(OpenBlockKind::Paragraph(vec![remainder]), Some(last));
                    let idx = self.arena.push(block);
                    self.open_stack.push(idx);
                }
            }
        } else {
            unreachable!()
        }
    }
}
