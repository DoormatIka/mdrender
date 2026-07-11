use crate::parser::blocks;

pub struct CommonmarkParser {
    start: usize,
    current: usize,
    phase_1_tokens: Vec<blocks::RawBlock>,
    phase_2_tokens: Vec<blocks::Block>,
}
impl CommonmarkParser {}
