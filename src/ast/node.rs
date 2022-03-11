use crate::token::token::Token;

pub enum Expr {
    Assign(Assign),
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
    Variable(Variable),
}

impl Expr {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) -> V::R {
        match self {
            Expr::Assign(assign) => visitor.visit_assign(assign),
            Expr::Binary(binary) => visitor.visit_binary(binary),
            Expr::Grouping(grouping) => visitor.visit_grouping(grouping),
            Expr::Literal(literal) => visitor.visit_literal(literal),
            Expr::Unary(unary) => visitor.visit_unary(unary),
            Expr::Variable(variable) => visitor.visit_variable(variable),
        }
    }
}

pub trait Visitor {
    type R;
    fn visit_assign (&mut self, assign: &Assign) -> Self::R;
    fn visit_binary (&mut self, binary: &Binary) -> Self::R;
    fn visit_grouping (&mut self, grouping: &Grouping) -> Self::R;
    fn visit_literal (&mut self, literal: &Literal) -> Self::R;
    fn visit_unary (&mut self, unary: &Unary) -> Self::R;
    fn visit_variable (&mut self, variable: &Variable) -> Self::R;
}

pub struct Assign {
    pub name: Token,
    pub value: Box<Expr>,
}

impl Assign {
    pub fn accept<T: Visitor> (&mut self, visitor: &mut T) -> T::R {
        visitor.visit_assign(self)
    }
}

pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Binary {
    pub fn accept<T: Visitor> (&mut self, visitor: &mut T) -> T::R {
        visitor.visit_binary(self)
    }
}

pub struct Grouping {
    pub expression: Box<Expr>,
}

impl Grouping {
    pub fn accept<T: Visitor> (&mut self, visitor: &mut T) -> T::R {
        visitor.visit_grouping(self)
    }
}

pub struct Literal {
    pub value: Token,
}

impl Literal {
    pub fn accept<T: Visitor> (&mut self, visitor: &mut T) -> T::R {
        visitor.visit_literal(self)
    }
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Unary {
    pub fn accept<T: Visitor> (&mut self, visitor: &mut T) -> T::R {
        visitor.visit_unary(self)
    }
}

pub struct Variable {
    pub name: Token,
}

impl Variable {
    pub fn accept<T: Visitor> (&mut self, visitor: &mut T) -> T::R {
        visitor.visit_variable(self)
    }
}

pub enum Stmt {
    Expression(Expression),
    Print(Print),
    Var(Var),
}

impl Stmt {
    pub fn accept<V: StmtVisitor>(&self, visitor: &mut V) -> V::R {
        match self {
            Stmt::Expression(expression) => visitor.visit_expression(expression),
            Stmt::Print(print) => visitor.visit_print(print),
            Stmt::Var(var) => visitor.visit_var(var),
        }
    }
}

pub trait StmtVisitor {
    type R;
    fn visit_expression (&mut self, expression: &Expression) -> Self::R;
    fn visit_print (&mut self, print: &Print) -> Self::R;
    fn visit_var (&mut self, var: &Var) -> Self::R;
}

pub struct Expression {
    pub expression: Box<Expr>,
}

impl Expression {
    pub fn accept<T: StmtVisitor> (&mut self, visitor: &mut T) -> T::R {
        visitor.visit_expression(self)
    }
}

pub struct Print {
    pub expression: Box<Expr>,
}

impl Print {
    pub fn accept<T: StmtVisitor> (&mut self, visitor: &mut T) -> T::R {
        visitor.visit_print(self)
    }
}

pub struct Var {
    pub name: Token,
    pub initializer: Option<Box<Expr>>,
}

impl Var {
    pub fn accept<T: StmtVisitor> (&mut self, visitor: &mut T) -> T::R {
        visitor.visit_var(self)
    }
}

