use crate::block::{
    blocks::{OpenBlock, OpenBlockKind},
    parser::BlockParser,
};

impl BlockParser {
    pub(in crate::block) fn check_indent(line: &str) -> Option<&str> {
        let indent = line.len() - line.trim_start_matches(' ').len();
        if indent > 3 {
            return None; // call indented code block instead for 4+ spaces.
        }
        Some(&line[indent..])
    }
    pub(in crate::block) fn push_child(&mut self, kind: OpenBlockKind, stays_open: bool) -> usize {
        let parent = *self.open_stack.last().unwrap();
        let mut block = OpenBlock::new(kind, Some(parent));
        block.is_open = stays_open;

        let idx = self.arena.push(block);
        if stays_open {
            self.open_stack.push(idx);
        }
        idx
    }
    pub(in crate::block) fn print_all_open_stack(&self, label: String) {
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
}
