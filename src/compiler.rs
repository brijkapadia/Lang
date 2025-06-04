use crate::ast;
use std::io::Write;
use std::fs::File;
use std::collections::HashMap;

pub struct Compiler{
    asm: File,
    stack_pos: usize,
    variable_map: HashMap<String, usize>
}

impl Compiler{
    pub fn new(name: &str) -> Compiler{
        Compiler{ asm: File::create(format!("{}.asm",name))
        .expect("Failed to create {name}.asm file"), stack_pos: 0, variable_map: HashMap::new()}
    }
    
    fn pre_write(&mut self){
        self.asm.write(
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
    mov rbp, rsp\n").expect("Failed to prewrite");
    
    }
    
    fn post_write(&mut self){
        self.asm.write(b"\tleave \n\tret").expect("Failed to postwrite");
    }
    fn write(&mut self, asm: &str){
        self.asm.write(("\t".to_owned() + asm+"\n").as_bytes()).expect("Failed to write");
    }
    
    fn push_eax_to_stack(&mut self){
        self.stack_pos += 4;
        self.write("sub rsp, 4");
        self.write(format!("mov [rbp-{}], eax",{self.stack_pos}).as_str());
    }
    pub fn compile(&mut self, program: ast::Program){
        self.pre_write();
    
        // where compiler starts; all enter + exits taken care of; starts in main: 
        self.evaluate_program(program);
    
        self.post_write();
    
    }
    fn generate_var_dec(&mut self,var: ast::VarDec){
        self.stack_pos +=  4;
        let stack_var = self.stack_pos;
        self.write("sub rsp, 4");
        self.variable_map.insert(var.name, stack_var);
        match var.value{
            Some(value) => {
                let expr = self.generate_expr(value); 
                match expr{
                    Some(n) => self.write(format!("mov dword [rbp-{}], {}",stack_var,n).as_str()),
                    None => self.write(format!("mov [rbp-{}], eax",stack_var).as_str())}
                },
            None => return
        }

    }

    fn generate_scope(&mut self, scope: ast::Scope){
        self.write("push rbp");
        self.write("mov rbp, rsp");

        for stmt in scope.stmts{
            self.evaluate_stmt(stmt);
        }

        self.write("leave");
    }

    fn evaluate_stmt(&mut self, stmt: ast::Stmt){
        match stmt{
                ast::Stmt::VarDec(var) => self.generate_var_dec(var),
                ast::Stmt::VarUpdate(var) => self.generate_var_update(var),
                ast::Stmt::Expr(_) => {} //TODO: expr in programs
                ast::Stmt::Scope(scope) => self.generate_scope(scope)
            }
    }

    fn evaluate_program(&mut self, program: ast::Program) {
        for stmt in program.body{
            self.evaluate_stmt(stmt);
        }

    }
    //return value goes in rax
    fn generate_var_update(&mut self, var: ast::VarUpdate){
        match self.generate_expr(var.value){
            None => self.write(format!("mov [rbp-{}], rax",self.variable_map.get(var.name.as_str()).unwrap()).as_str()),
            Some(n) => self.write(format!("mov [rbp-{}], {}",self.variable_map.get(var.name.as_str()).unwrap(),n).as_str())
        }
        

    }
    fn generate_expr_in_rax(&mut self, expr: ast::Expr){
        let result = self.generate_expr(expr);
        match result {
            Some(n) => self.write(format!("mov rax, {}",n).as_str()),
            None => {}
            
        }
    }
    fn generate_expr(&mut self, expr: ast::Expr)->Option<i64> {
        match expr{
            ast::Expr::IntLiteral(node) => Some(node.value),
            ast::Expr::BinaryExpr(node)=> {return self.generate_binary_expr(node);}
            ast::Expr::VarLiteral(var) => {self.generate_var_expr(var); return None},
            ast::Expr::FloatLiteral(_) => todo!()
            
        }
    }

    fn generate_var_expr(&mut self, var: ast::VarLiteral){
        self.write(format!("mov rax, [rbp-{}]",self.variable_map[var.name.as_str()]).as_str());
    }

    fn generate_binary_expr(&mut self, expr: ast::BinaryExpr)->Option<i64>{
        let right_result = self.generate_expr(*expr.right);// this is in eax
        let mut right_evaluated = false;
        let mut right_number: i64 = 0;
        match right_result{
            None => {
                self.write("mov rcx, rax");
            },
            Some(n) => {
                right_number = n;
                right_evaluated = true;
            }
        }
        let left_result = self.generate_expr(*expr.left); //this is in eax and can stay
        let mut left_evaluated = false;
        let mut left_number: i64 = 0;
        match left_result{
            None => {
            },
            Some(n) => {
                left_number = n;
                left_evaluated = true;
            }
        }

        if right_evaluated && left_evaluated{
            match expr.op.as_str(){
                "+" => return Some(left_number + right_number),
                "-" => return Some(left_number - right_number),
                "*" => return Some(left_number * right_number),
                _ => {return None;}

            }
        } else if right_evaluated{
            match expr.op.as_str(){
                "+" => self.write(format!("add rax, {}",right_number).as_str()),
                "-" => self.write(format!("sub rax, {}",right_number).as_str()),
                "*" => self.write(format!("mul {}",right_number).as_str()), 
                _ => {}
            }
            return None
        } else if left_evaluated{
            self.write("mov rax, rcx");
            match expr.op.as_str(){
                "+" => self.write(format!("add rax, {}",right_number).as_str()),
                "-" => self.write(format!("sub rax, {}",right_number).as_str()),
                "*" => self.write(format!("mul {}",right_number).as_str()), 
                _ => {}
            }
            return None
        }else{
        match expr.op.as_str(){
            "+" => self.write("add rax, rcx"),
            "-" => self.write("sub rax, rcx"),
            "*" => self.write("mul rcx"),
            _ => {} 
        }
        return None
    }
    }
}

