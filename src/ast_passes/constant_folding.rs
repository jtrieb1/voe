use crate::parser::{
    atom::{Atom, AtomValue},
    Block, Conditional, Expression, FunctionDefinition, Operator, Program, Statement,
    VariableDeclaration,
};

use super::ASTPass;

#[derive(Debug, Clone, Copy)]
pub struct ConstantFolding;

impl ConstantFolding {
    fn fold_statement(&self, statement: Statement) -> Statement {
        match statement {
            Statement::Function(fd) => self.fold_function_definition(fd),
            Statement::VariableDeclaration(vd) => self.fold_variable_declaration(vd),
            Statement::Expression(expr) => Statement::expression(self.fold_expression(expr)),
            Statement::Conditional(cond) => self.fold_conditional(cond),
        }
    }

    fn fold_function_definition(&self, fd: FunctionDefinition) -> Statement {
        let FunctionDefinition {
            name,
            inputs,
            return_type,
            body,
        } = fd;
        let processed_body: Vec<Statement> = body
            .statements
            .into_iter()
            .map(|s| self.fold_statement(s))
            .collect();
        Statement::Function(FunctionDefinition::new(
            name,
            inputs,
            return_type,
            Block::new(processed_body),
        ))
    }

    fn fold_variable_declaration(&self, vd: VariableDeclaration) -> Statement {
        let VariableDeclaration {
            name,
            var_type,
            value,
        } = vd;
        let mut ty = var_type;
        let processed_value = value.map(|expr| self.fold_expression(expr));
        if let Some(expr) = &processed_value {
            ty = expr.return_type();
            if let Expression::Atom(atom) = expr {
                if let AtomValue::Integer(value) = atom.value {
                    return Statement::VariableDeclaration(VariableDeclaration::new(
                        name,
                        ty.clone(),
                        Some(Expression::Atom(Atom::from_i128(value, ty))),
                    ));
                } else if let AtomValue::Float(value) = atom.value {
                    return Statement::VariableDeclaration(VariableDeclaration::new(
                        name,
                        ty.clone(),
                        Some(Expression::Atom(Atom::from_f64(value, ty))),
                    ));
                }
            }
        }
        Statement::VariableDeclaration(VariableDeclaration::new(name, ty, processed_value))
    }

    fn fold_numeric_op(&self, lhs: Atom, op: Operator, rhs: Atom) -> Expression {
        if let Some(lty) = &lhs.ty {
            if let Some(rty) = &rhs.ty {
                if lty.is_integral() && rty.is_integral() {
                    let Atom {
                        negative: lhs_neg,
                        value: lhs_val,
                        ty: _,
                    } = &lhs;
                    let Atom {
                        negative: rhs_neg,
                        value: rhs_val,
                        ty: _,
                    } = &rhs;

                    let AtomValue::Integer(lhs_val) = lhs_val else {
                        return Expression::BinaryOperation(
                            Box::new(Expression::Atom(lhs)),
                            op,
                            Box::new(Expression::Atom(rhs)),
                        );
                    };
                    let AtomValue::Integer(rhs_val) = rhs_val else {
                        return Expression::BinaryOperation(
                            Box::new(Expression::Atom(lhs)),
                            op,
                            Box::new(Expression::Atom(rhs)),
                        );
                    };

                    let lhs_val = if *lhs_neg { -lhs_val } else { *lhs_val };
                    let rhs_val = if *rhs_neg { -rhs_val } else { *rhs_val };

                    match op {
                        Operator::Add => {
                            Expression::Atom(Atom::from_i128(lhs_val + rhs_val, lty.join(rty)))
                        }
                        Operator::Subtract => {
                            Expression::Atom(Atom::from_i128(lhs_val - rhs_val, lty.join(rty)))
                        }
                        Operator::Multiply => {
                            Expression::Atom(Atom::from_i128(lhs_val * rhs_val, lty.join(rty)))
                        }
                        Operator::Divide => {
                            Expression::Atom(Atom::from_i128(lhs_val / rhs_val, lty.join(rty)))
                        }
                        Operator::Modulo => {
                            Expression::Atom(Atom::from_i128(lhs_val % rhs_val, lty.join(rty)))
                        }
                        _ => Expression::BinaryOperation(
                            Box::new(Expression::Atom(lhs)),
                            op,
                            Box::new(Expression::Atom(rhs)),
                        ),
                    }
                } else if lty.is_decimal() && rty.is_decimal() {
                    // Implement decimal operations here
                    let Atom {
                        negative: lhs_neg,
                        value: lhs_val,
                        ty: _,
                    } = &lhs;
                    let Atom {
                        negative: rhs_neg,
                        value: rhs_val,
                        ty: _,
                    } = &rhs;

                    let AtomValue::Float(lhs_val) = lhs_val else {
                        return Expression::BinaryOperation(
                            Box::new(Expression::Atom(lhs)),
                            op,
                            Box::new(Expression::Atom(rhs)),
                        );
                    };
                    let AtomValue::Float(rhs_val) = rhs_val else {
                        return Expression::BinaryOperation(
                            Box::new(Expression::Atom(lhs)),
                            op,
                            Box::new(Expression::Atom(rhs)),
                        );
                    };

                    let lhs_val = if *lhs_neg { -lhs_val } else { *lhs_val };
                    let rhs_val = if *rhs_neg { -rhs_val } else { *rhs_val };

                    match op {
                        Operator::Add => {
                            Expression::Atom(Atom::from_f64(lhs_val + rhs_val, lty.join(rty)))
                        }
                        Operator::Subtract => {
                            Expression::Atom(Atom::from_f64(lhs_val - rhs_val, lty.join(rty)))
                        }
                        Operator::Multiply => {
                            Expression::Atom(Atom::from_f64(lhs_val * rhs_val, lty.join(rty)))
                        }
                        Operator::Divide => {
                            Expression::Atom(Atom::from_f64(lhs_val / rhs_val, lty.join(rty)))
                        }
                        Operator::Modulo => {
                            Expression::Atom(Atom::from_f64(lhs_val % rhs_val, lty.join(rty)))
                        }
                        _ => Expression::BinaryOperation(
                            Box::new(Expression::Atom(lhs)),
                            op,
                            Box::new(Expression::Atom(rhs)),
                        ),
                    }
                } else {
                    Expression::BinaryOperation(
                        Box::new(Expression::Atom(lhs)),
                        op,
                        Box::new(Expression::Atom(rhs)),
                    )
                }
            } else {
                Expression::BinaryOperation(
                    Box::new(Expression::Atom(lhs)),
                    op,
                    Box::new(Expression::Atom(rhs)),
                )
            }
        } else {
            Expression::BinaryOperation(
                Box::new(Expression::Atom(lhs)),
                op,
                Box::new(Expression::Atom(rhs)),
            )
        }
    }

    fn fold_expression(&self, expr: Expression) -> Expression {
        match expr {
            Expression::Atom(atom) => {
                let Atom {
                    negative,
                    value,
                    ty,
                } = atom;
                match value {
                    AtomValue::ParExpr(einner) => Expression::Atom(Atom::new(
                        negative,
                        AtomValue::ParExpr(Box::new(self.fold_expression(*einner))),
                        ty,
                    )),
                    _ => Expression::Atom(Atom {
                        negative,
                        value,
                        ty,
                    }),
                }
            }
            Expression::BinaryOperation(lhs, op, rhs) => {
                let lhs = self.fold_expression(*lhs);
                let rhs = self.fold_expression(*rhs);
                match (lhs.clone(), op, rhs.clone()) {
                    (Expression::Atom(lhs_atom), op, Expression::Atom(rhs_atom)) => {
                        if let Some(lty) = lhs_atom.get_type() {
                            if let Some(rty) = rhs_atom.get_type() {
                                if lty.is_integral() && rty.is_integral()
                                    || lty.is_decimal() && rty.is_decimal()
                                {
                                    self.fold_numeric_op(lhs_atom, op, rhs_atom)
                                } else {
                                    Expression::BinaryOperation(Box::new(lhs), op, Box::new(rhs))
                                }
                            } else {
                                Expression::BinaryOperation(Box::new(lhs), op, Box::new(rhs))
                            }
                        } else {
                            Expression::BinaryOperation(Box::new(lhs), op, Box::new(rhs))
                        }
                    }
                    (lhs, op, rhs) => Expression::BinaryOperation(Box::new(lhs), op, Box::new(rhs)),
                }
            }
        }
    }

    fn fold_conditional(&self, cond: Conditional) -> Statement {
        let Conditional {
            condition,
            then_block,
            else_block,
        } = cond;
        let processed_condition = self.fold_expression(condition);
        let processed_then_block: Vec<Statement> = then_block
            .statements
            .into_iter()
            .map(|s| self.fold_statement(s))
            .collect();
        let processed_else_block: Option<Vec<Statement>> = else_block.map(|block| {
            block
                .statements
                .into_iter()
                .map(|s| self.fold_statement(s))
                .collect()
        });
        Statement::Conditional(Conditional::new(
            processed_condition,
            Block::new(processed_then_block),
            processed_else_block.map(Block::new),
        ))
    }
}

impl ASTPass for ConstantFolding {
    fn run(&mut self, program: Program) -> Program {
        let mut processed_statements = Vec::new();
        for statement in program.statements {
            processed_statements.push(self.fold_statement(statement));
        }
        Program::new(processed_statements)
    }
}
