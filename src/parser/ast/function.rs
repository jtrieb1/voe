use super::{Block, Type, VariableDeclaration};

#[derive(PartialEq, Debug, Clone)]
pub struct FunctionDefinition {
    pub name: String,
    pub inputs: Vec<VariableDeclaration>,
    pub return_type: Type,
    pub body: Block,
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

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn inputs(&self) -> &Vec<VariableDeclaration> {
        &self.inputs
    }

    pub fn return_type(&self) -> &Type {
        &self.return_type
    }

    pub fn body(&self) -> &Block {
        &self.body
    }
}

impl std::fmt::Display for FunctionDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "fn {}({}) -> {} {}",
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
