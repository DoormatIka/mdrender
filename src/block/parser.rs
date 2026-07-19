use crate::block::blocks::{Arena, Document, OpenBlock, OpenBlockKind, RawBlock};

pub struct BlockParser {
    pub(in crate::block) arena: Arena,
    pub(in crate::block) open_stack: Vec<usize>,
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
            let Some(node) = self.arena.get(idx) else {
                continue;
            };

            let kind = &node.kind;
            let matched = match kind {
                OpenBlockKind::BlockQuote => Self::match_block_quote(remaining),
                OpenBlockKind::Paragraph(_) => Self::match_paragraph(remaining),
                OpenBlockKind::Document
                | OpenBlockKind::ThematicBreak
                | OpenBlockKind::Heading { .. } => {
                    continue;
                }
                _ => todo!(),
            };

            match matched {
                Some(r) => remaining = r,
                None => return (depth, remaining),
            }
        }

        (self.open_stack.len(), remaining)
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
    fn try_open_new_blocks<'a>(&mut self, remainder: &'a str) -> &'a str {
        let mut rest = remainder;

        loop {
            if let Some(rem) = Self::match_block_quote(rest) {
                self.push_child(OpenBlockKind::BlockQuote, true);
                rest = rem;

                continue;
            }
            if let Some((level, raw)) = Self::match_atx_heading(rest) {
                let heading = OpenBlockKind::Heading {
                    level,
                    raw: raw.to_string(),
                };
                self.push_child(heading, false);
                rest = "";

                break;
            }
            if Self::match_thematic_break(rest).is_some() {
                self.push_child(OpenBlockKind::ThematicBreak, false);
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
