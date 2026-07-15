use crate::parser::blocks::{Arena, Document, OpenBlock, OpenBlockKind, RawBlock};

pub struct BlockParser {
    pub arena: Arena,
    pub open_stack: Vec<usize>,
}
impl BlockParser {
    pub fn new() -> Self {
        let mut arena = Arena::new();
        let root = arena.push(OpenBlock::new(OpenBlockKind::Document, None));
        Self {
            arena,
            open_stack: vec![root],
        }
    }

    pub fn parse_phase_one(&mut self, input: String) -> Document<RawBlock> {
        let doc = Document::<RawBlock>::new();

        for line in input.lines() {
            self.process_line(line);
        }
        self.close_blocks_below(0); // to close the document struct

        doc
    }

    fn process_line(&mut self, line: &str) {
        let (depth, remainder) = self.match_open_blocks(line);
        self.close_blocks_below(depth);
        self.try_open_new_blocks(remainder, depth);
        self.append_text(remainder);
    }

    // checking for any open blocks, nothing inserted yet.
    // basically eating through the matched
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
                    _ => todo!(),
                }
            }
        }

        (self.open_stack.len(), remaining)
    }

    fn match_paragraph(line: &str) -> Option<&str> {
        if line.trim().is_empty() {
            None
        } else {
            Some(line)
        }
    }
    fn match_block_quote(line: &str) -> Option<&str> {
        let indent = line.len() - line.trim_start_matches(' ').len();
        if indent > 3 {
            return None; // call indented code block instead for 4+ spaces.
        }
        let trimmed = &line[indent..];

        trimmed
            .strip_prefix('>')
            .map(|r| r.strip_prefix(' ').unwrap_or(r))
    }

    /// closing
    fn close_blocks_below(&mut self, depth: usize) {
        while self.open_stack.len() > depth {
            let idx = self.open_stack.pop().unwrap();
            self.arena.get_mut(idx).unwrap().is_open = false;
            // not handling lazy continuation yet.
        }
    }

    /// opening/creating new blocks
    fn try_open_new_blocks<'a>(&'a mut self, remainder: &'a str, depth: usize) -> &'a str {
        let mut rest = remainder;

        loop {
            if let Some(rem) = Self::match_block_quote(rest) {
                println!("block quote: {}", rem);
                let parent = *self.open_stack.last().unwrap();
                let block = OpenBlock::new(OpenBlockKind::BlockQuote, Some(parent));
                let idx = self.arena.push(block);

                self.open_stack.push(idx);
                rest = rem;

                continue;
            }

            break;
        }

        rest
    }

    // just appending the existing text as a fallback
    fn append_text<'a>(&mut self, remainder: &'a str) {
        let last = *self.open_stack.last().unwrap();
        let remainder = remainder.to_string();

        if let Some(block) = self.arena.get_mut(last) {
            match &mut block.kind {
                OpenBlockKind::Paragraph(p) => {
                    p.push(remainder.to_string());
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
