use crate::ast::{BinaryOperator, Command, Expression, Statement};
use crate::interpreter::{Interpreter, NextAction, Result};
use crate::vm::{Op, Vm};

pub struct BytecodeInterpreter {
    vm: Vm,
}

impl BytecodeInterpreter {
    pub fn new() -> Self {
        Self { vm: Vm::new() }
    }

    fn compile_expression(&mut self, expr: &Expression) -> Result<()> {
        match expr {
            Expression::Scalar(n) => {
                let index = self.vm.add_constant(n.to_f64());
                self.vm.add_op_with_arg(Op::PushConstant, index);
            }
            Expression::Identifier(_) => todo!(),
            Expression::Negate(rhs) => {
                self.compile_expression(rhs)?;
                self.vm.add_op(Op::Negate);
            }
            Expression::BinaryOperator(op, lhs, rhs) => {
                self.compile_expression(lhs)?;
                self.compile_expression(rhs)?;

                match op {
                    BinaryOperator::Add => {
                        self.vm.add_op(Op::Add);
                    }
                    BinaryOperator::Sub => todo!(),
                    BinaryOperator::Mul => todo!(),
                    BinaryOperator::Div => todo!(),
                    BinaryOperator::ConvertTo => todo!(),
                }
            }
        };
        Ok(())
    }

    fn compile_statement(&mut self, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::Expression(expr) => {
                self.compile_expression(&expr)?;
            }
            Statement::Command(Command::List) => {
                todo!();
            }
            Statement::Command(Command::Quit) => {
                todo!();
            }
            Statement::Assignment(_, _) => {
                todo!();
            }
        }

        self.vm.add_op(Op::Print);
        self.vm.add_op(Op::Exit);

        Ok(())
    }

    fn run(&mut self) {
        self.vm.run();
    }
}

impl Interpreter for BytecodeInterpreter {
    fn interpret(&mut self, stmt: &Statement) -> Result<NextAction> {
        self.compile_statement(stmt)?;
        self.run();

        Ok(NextAction::Continue)
    }
}