use crate::parser::blocks::{self, Block, Document, Inline, RawBlock};

pub struct CommonmarkLexer {
    lexems: Vec<String>,
    index: usize,
}
impl CommonmarkLexer {
    fn new(expr: String, separator: String) -> Self {
        let lexems = expr.split(&separator).map(str::to_string).collect();
        Self {
            lexems: lexems,
            index: 0,
        }
    }
    fn current(&self) -> Option<String> {
        self.lexems.get(self.index).cloned()
    }
    fn next(&mut self) -> Option<String> {
        if self.isAtEnd() {
            return None;
        }
        let token = self.lexems[self.index].clone();
        self.index += 1;
        return Some(token);
    }
    fn isAtEnd(&self) -> bool {
        self.index >= self.lexems.len()
    }
}

pub struct CommonmarkParser {
    phase_1_tokens: Vec<blocks::RawBlock>,
    phase_2_tokens: Vec<blocks::Block>,
}
impl CommonmarkParser {
    pub fn new() -> Self {
        Self {
            phase_1_tokens: Vec::new(),
            phase_2_tokens: Vec::new(),
        }
    }

    pub fn parse(&self, input: String) -> Document {
        let mut doc = Document::new();
        let mut lexer = CommonmarkLexer::new(input, "\n\n".to_string());

        while let Some(block) = lexer.next() {
            dbg!(block);
        }

        doc
    }
    fn parseInline(block: Block) -> Inline {
        todo!();
    }
}
