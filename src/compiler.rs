use crate::ast;
use crate::parser::{Atom, BinaryExpr, Expr, OperationType};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

pub struct Compiler {
    asm: File,
    stack_pos: usize,
    variable_map: HashMap<String, usize>,
}

impl Compiler {
    pub fn new(name: &str) -> Compiler {
        Compiler {
            asm: File::create(format!("{}.asm", name)).expect("Failed to create {name}.asm file"),
            stack_pos: 0,
            variable_map: HashMap::new(),
        }
    }

    fn pre_write(&mut self) {
        self.asm
            .write(
                b"global _start
section .text

_start:
    call main
    call exit

exit:
    mov rax, 60
    mov rdi, 0
    syscall

main:
    push rbp
    mov rbp, rsp\n",
            )
            .expect("Failed to prewrite");
    }

    fn post_write(&mut self) {
        self.asm
            .write(b"\tleave \n\tret")
            .expect("Failed to postwrite");
    }
    fn write(&mut self, asm: &str) {
        self.asm
            .write(("\t".to_owned() + asm + "\n").as_bytes())
            .expect("Failed to write");
    }

    fn push_eax_to_stack(&mut self) {
        self.stack_pos += 4;
        self.write("sub rsp, 4");
        self.write(format!("mov [rbp-{}], eax", { self.stack_pos }).as_str());
    }
    pub fn compile(&mut self, program: ast::Scope) {
        self.pre_write();

        // where compiler starts; all enter + exits taken care of; starts in main:
        self.evaluate_program(program);

        self.post_write();
    }
    fn evaluate_program(&mut self, scope: ast::Scope) {
        for stmt in scope.body {
            self.evaluate_stmt(stmt);
        }
    }
    fn evaluate_stmt(&mut self, stmt: ast::Stmt) {
        match stmt {
            ast::Stmt::VariableDeclaration(var) => self.generate_var_dec(var),
            ast::Stmt::VariableUpdate(var) => self.generate_var_update(var),
            ast::Stmt::Expr(_) => {} //TODO: expression in programs
            ast::Stmt::Scope(scope) => self.generate_scope(scope),
            ast::Stmt::While(w) => self.generate_while(w),
            _ => todo!(),
        }
    }

    fn generate_while(&mut self, _: ast::While) {
        todo!()
    }

    fn generate_var_dec(&mut self, variable_declaration: ast::VariableDeclaration) {
        let size = variable_declaration.data.variable_bytes;
        self.stack_pos += size;
        let variable_position_in_stack = self.stack_pos;
        variable_declaration.data.stack_position.set(self.stack_pos);
        self.write(&format!("sub rsp, {}", size));
        let expression = self.generate_expr(variable_declaration.value);
        match expression {
            Some(n) => self
                .write(format!("mov dword [rbp-{}], {}", variable_position_in_stack, n).as_str()),
            Option::None => {
                self.write(format!("mov [rbp-{}], eax", variable_position_in_stack).as_str())
            }
        }
    }

    fn generate_var_update(&mut self, variable: ast::VariableUpdate) {
        match self.generate_expr(variable.value) {
            Option::None => self.write(&format!(
                "mov [rbp-{}], rax",
                variable.data.stack_position.get()
            )),
            Some(n) => self.write(&format!(
                "mov [rbp-{}], {}",
                variable.data.stack_position.get(),
                n
            )),
        }
    }
    fn generate_scope(&mut self, scope: ast::Scope) {
        self.write("push rbp");
        self.write("mov rbp, rsp");

        for stmt in scope.body {
            self.evaluate_stmt(stmt);
        }
        self.write("leave");
    }
    fn generate_expr(&mut self, expr: Expr) -> Option<i32> {
        match expr {
            Expr::Atom(Atom::IntLiteral(node)) => Some(node),
            Expr::BinaryExpr(node) => {
                return self.generate_binary_expr(node);
            }
            Expr::Atom(Atom::VariableLiteral(var)) => {
                self.generate_var_expr(&var);
                return None;
            }
            Expr::Atom(Atom::BoolLiteral(bool)) => Some(bool as i32),
            _ => todo!(),
        }
    }

    //return value goes in rax

    fn generate_expr_in_rax(&mut self, expr: Expr) {
        let result = self.generate_expr(expr);
        match result {
            Some(n) => self.write(format!("mov rax, {}", n).as_str()),
            Option::None => {}
        }
    }

    fn generate_var_expr(&mut self, var: &str) {
        self.write(format!("mov rax, [rbp-{}]", self.variable_map[var]).as_str());
    }

    fn generate_binary_expr(&mut self, expr: BinaryExpr) -> Option<i32> {
        let right_result = self.generate_expr(*expr.right); // this is in eax
        let mut right_evaluated = false;
        let mut right_number: i32 = 0;
        match right_result {
            None => {
                self.write("mov rcx, rax");
            }
            Some(n) => {
                right_number = n;
                right_evaluated = true;
            }
        }
        let left_result = self.generate_expr(*expr.left); //this is in eax and can stay
        let mut left_evaluated = false;
        let mut left_number: i32 = 0;
        match left_result {
            None => {}
            Some(n) => {
                left_number = n;
                left_evaluated = true;
            }
        }

        if right_evaluated && left_evaluated {
            match expr.op {
                OperationType::Add => return Some(left_number + right_number),
                OperationType::Subtract => return Some(left_number - right_number),
                OperationType::Multiply => return Some(left_number * right_number),
                _ => todo!(),
            }
        } else if right_evaluated {
            match expr.op {
                OperationType::Add => self.write(format!("add rax, {}", right_number).as_str()),
                OperationType::Subtract => {
                    self.write(format!("sub rax, {}", right_number).as_str())
                }
                OperationType::Multiply => self.write(format!("mul {}", right_number).as_str()),
                _ => todo!(),
            }
            return None;
        } else if left_evaluated {
            self.write("mov rax, rcx");
            match expr.op {
                OperationType::Add => self.write(format!("add rax, {}", right_number).as_str()),
                OperationType::Subtract => {
                    self.write(format!("sub rax, {}", right_number).as_str())
                }
                OperationType::Multiply => self.write(format!("mul {}", right_number).as_str()),
                _ => {}
            }
            return None;
        } else {
            match expr.op {
                OperationType::Add => self.write("add rax, rcx"),
                OperationType::Subtract => self.write("sub rax, rcx"),
                OperationType::Multiply => self.write("mul rcx"),
                _ => {}
            }
            return None;
        }
    }
}
