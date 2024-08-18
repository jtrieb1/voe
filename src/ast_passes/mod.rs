use crate::parser::Program;

mod constant_folding;
pub use constant_folding::ConstantFolding;

pub trait ASTPass {
    fn run(&mut self, program: Program) -> Program;
}
