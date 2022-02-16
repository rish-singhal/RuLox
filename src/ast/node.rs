use crate::token::token::Token;

pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

pub trait Visitor {
    type R;
    fn visit_binary (&self, binary: &Binary) -> Self::R;
    fn visit_grouping (&self, grouping: &Grouping) -> Self::R;
    fn visit_literal (&self, literal: &Literal) -> Self::R;
    fn visit_unary (&self, unary: &Unary) -> Self::R;
}

pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Binary {
   pub fn accept<T: Visitor> (&self, visitor: &mut T) -> T::R {
       return visitor.visit_binary(&self);
   }
}

pub struct Grouping {
    pub expression: Box<Expr>,
}

impl Grouping {
   pub fn accept<T: Visitor> (&self, visitor: &mut T) -> T::R {
       return visitor.visit_grouping(&self);
   }
}

pub struct Literal {
    pub value: Token,
}

impl Literal {
   pub fn accept<T: Visitor> (&self, visitor: &mut T) -> T::R {
       return visitor.visit_literal(&self);
   }
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Unary {
   pub fn accept<T: Visitor> (&self, visitor: &mut T) -> T::R {
       return visitor.visit_unary(&self);
   }
}
