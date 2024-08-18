use super::{Block, Type, VariableDeclaration};

#[derive(PartialEq, Debug, Clone)]
pub struct FunctionDefinition {
    name: String,
    inputs: Vec<VariableDeclaration>,
    return_type: Type,
    body: Block,
}

impl FunctionDefinition {
    pub fn new(
        name: String,
        inputs: Vec<VariableDeclaration>,
        return_type: Type,
        body: Block,
    ) -> FunctionDefinition {
        FunctionDefinition {
            name,
            inputs,
            return_type,
            body,
        }
    }
}

impl std::fmt::Display for FunctionDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "function {}({}) -> {} {}",
            self.name,
            self.inputs
                .iter()
                .map(|i| format!("{}", i))
                .collect::<Vec<_>>()
                .join(", "),
            self.return_type,
            self.body
        )
    }
}
