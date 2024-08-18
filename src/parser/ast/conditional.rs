use super::{Block, Expression};

#[derive(PartialEq, Debug, Clone)]
pub struct Conditional {
    pub condition: Expression,
    pub then_block: Block,
    pub else_block: Option<Block>,
}

impl Conditional {
    pub fn new(condition: Expression, then_block: Block, else_block: Option<Block>) -> Conditional {
        Conditional {
            condition,
            then_block,
            else_block,
        }
    }
}

impl std::fmt::Display for Conditional {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "if {} {}{}",
            self.condition,
            self.then_block,
            match &self.else_block {
                Some(block) => format!(" else {}", block),
                None => "".to_string(),
            }
        )
    }
}
