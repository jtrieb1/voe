use super::{Expression, Type};

#[derive(PartialEq, Debug, Clone)]
pub struct VariableDeclaration {
    pub name: String,
    pub var_type: Option<Type>,
    pub value: Option<Expression>,
}

impl VariableDeclaration {
    pub fn new(name: String, var_type: Option<Type>, value: Option<Expression>) -> VariableDeclaration {
        VariableDeclaration {
            name,
            var_type,
            value,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn var_type(&self) -> &Option<Type> {
        &self.var_type
    }
}

impl std::fmt::Display for VariableDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "let {}{}{}",
            self.name,
            match self.var_type() {
                Some(ty) => format!(": {}", ty),
                None => "".to_string(),
            },
            match &self.value {
                Some(expr) => format!(" = {}", expr),
                None => "".to_string(),
            }
        )
    }
}