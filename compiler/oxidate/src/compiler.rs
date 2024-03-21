use std::{error::Error, fmt::Display};
use anyhow::Result;

use parser::{BlockSeq, Decl, Expr, Parser};
use bytecode::ByteCode;

pub struct Compiler {
    bytecode: Vec<ByteCode>,
    program: BlockSeq
}

#[derive(Debug, PartialEq)]
pub struct CompileError {
    msg: String,
}

impl CompileError {
    pub fn new(err: &str) -> CompileError {
        CompileError {
            msg: err.to_owned(),
        }
    }
}

impl Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[CompileError]: {}", self.msg)
    }
}

impl std::error::Error for CompileError {}

impl Compiler {
    pub fn new(program: BlockSeq) -> Compiler {
        Compiler {
            bytecode: vec![],
            program
        }
    }

    pub fn compile_expr(expr:&Expr) -> Result<ByteCode, CompileError> {
        match expr {
            Expr::Integer(val) => Ok(ByteCode::ldc(*val)),
            Expr::Float(val) => Ok(ByteCode::ldc(*val)),
            Expr::Bool(val) => Ok(ByteCode::ldc(*val)),
            _ => unimplemented!()
        }
    }

    fn compile_decl(decl: Decl) -> Result<ByteCode,CompileError> {
        let code = match decl {
            Decl::ExprStmt(expr) => {
                Compiler::compile_expr(&expr)
            },
            _ => unimplemented!()
            // Decl::LetStmt(stmt) => {
            //     Ok(ByteCode::DONE)

            // },
            // Decl::Block(blk) => {
            //     Ok(ByteCode::DONE)
            // }
        }?;

        Ok(code)
    }

    pub fn compile(self) -> anyhow::Result<Vec<ByteCode>, CompileError>{
        // println!("Compile");
        let mut bytecode: Vec<ByteCode> = vec![];
        let decls = self.program.decls;

        for decl in decls {
            let code = Compiler::compile_decl(decl)?;
            bytecode.push(code);
            // pop result of statements - need to ensure all stmts produce something (either Unit or something else)
            bytecode.push(ByteCode::POP); 
        }

        // Handle expr
        if let Some(expr) = self.program.last_expr {
            let code = Compiler::compile_expr(expr.as_ref())?;
            bytecode.push(code);
        }

        bytecode.push(ByteCode::DONE);

        Ok(bytecode)
    }
}

#[cfg(test)]
mod tests {

    use bytecode::ByteCode;
    use bytecode::ByteCode::*;
    use parser::Parser;

    use super::Compiler;

    fn exp_compile_str(inp:&str) -> Vec<ByteCode>{
        let parser = Parser::new_from_string(inp);
        let parsed = parser.parse().expect("Should parse");
        let comp = Compiler::new(parsed);
        comp.compile().expect("Should compile")
    }

    #[test]
    fn test_compile_simple() {
        let res = exp_compile_str("42;");
        assert_eq!(res, vec![ByteCode::ldc(42), POP, DONE]);

        let res = exp_compile_str("42; 45; 30");
        assert_eq!(res, vec![ByteCode::ldc(42), POP, ByteCode::ldc(45), POP, ByteCode::ldc(30), DONE]);

        let res = exp_compile_str("42; true; 2.36;");
        assert_eq!(res, vec![ByteCode::ldc(42), POP, ByteCode::ldc(true), POP, ByteCode::ldc(2.36), POP, DONE])
    }
}
