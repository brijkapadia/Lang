use crate::tokens;
use crate::ast;

pub struct ProgramIter<'a>{
    program: &'a ast::Program,
    index: usize
}

impl<'a> ProgramIter<'a>{
    pub fn new(program: &'a ast::Program) -> Self{
        Self{program, index: 0}
    }
}

impl<'a> Iterator for ProgramIter<'a>{
    type Item = &'a ast::Stmt;
    fn next(&mut self) -> Option<Self::Item> {
        let result = &self.program.body[self.index];
        self.index += 1;
        Some(result)
    }
}


pub fn print_token_list(token_list: &tokens::TokenList){
    println!("[");
    let token_iter = tokens::TokenListIter::new(token_list);

    for token in token_iter{
        println!("\t[value: \"{}\", token_type: {:?}]",&token.value, token.token_type);
    }
    println!("]");
}

pub fn print_parse_tree(program: &ast::Program){
    let printer = Printer {};
    printer.print_parsed(&program);
}

struct Printer;

impl Printer{
    fn print_binary_expr(&self, bin: &ast::BinaryExpr){
        println!("[Kind: BinaryExpr"); 
        println!("Left: ");
        self.print_expr(&*bin.left);
        println!("Right: ");
        self.print_expr(&*bin.right);
        println!("Op: {}]",bin.op );
    }
    fn print_num_expr(&self, num: &ast::IntLiteral){
        println!("[Kind: NumericLiteral, Value: {}]",num.value);
    }
    fn print_var_expr(&self, var: &ast::VarLiteral){
        println!("[Kind: VarLiteral, Value {}]",var.name);
    }
    fn print_expr(&self, expr: &ast::Expr){
        match expr{
            ast::Expr::BinaryExpr(bin) => self.print_binary_expr(bin),
            ast::Expr::IntLiteral(num) => self.print_num_expr(num),
            ast::Expr::FloatLiteral(num) => todo!(),
            ast::Expr::VarLiteral(var) => self.print_var_expr(var)
        }
    }
    fn print_var(&self, var: &ast::VarDec){
        println!("[Kind: VarDec, Name: {}, Value: ", var.name);
        self.print_expr(var.value.as_ref().unwrap());
        println!("]")
    }
    fn print_update(&self, var: &ast::VarUpdate){
        println!("[Kind: VarUpdate, Name: {}, value: ", var.name);
        self.print_expr(&var.value);
        println!("]")
    }
    fn print_parsed(&self, program: &ast::Program){
        let program_iter = ProgramIter::new(program);
        for stmt in program_iter{
            match stmt{
                ast::Stmt::Expr(expr) => self.print_expr(expr),
                ast::Stmt::VarDec(var) => self.print_var(var),
                ast::Stmt::VarUpdate(var) => self.print_update(var),
                ast::Stmt::Scope(scope) => todo!()
            }
        }
    }
}

