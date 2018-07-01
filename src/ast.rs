extern crate im;

use im::conslist::ConsList;
use im::ordmap::OrdMap;

use std::convert::From;

// To be replaced with some "BigNum" type
type NumberType = i64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Atom<'a> {
    Sym(&'a str),
    Num(NumberType),
    // Bool, Str, etc.
}

impl<'a> From<&'a str> for Atom<'a> {
    fn from(inp: &'a str) -> Self {
        if let Ok(n) = inp.parse::<NumberType>() {
            return Atom::Num(n);
        }
        Atom::Sym(inp)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr<'a> {
    SExpr(ConsList<Expr<'a>>),
    Atom(Atom<'a>)
}

impl<'a> From<Vec<Expr<'a>>> for Expr<'a> {
    fn from(vec: Vec<Expr<'a>>) -> Self {
        let list: ConsList<Expr<'a>> = vec.iter().collect();
        Expr::SExpr(list)
    }
}

/// Environment for symbol lookup.
pub type Env<'a> = OrdMap<i32, Expr<'a>>;

impl<'a> Expr<'a> {

    pub fn eval(&mut self, envi: Env<'a>) -> (Expr<'a>, Env<'a>) {
        //let res = match *self {
        //    SExpr(l) => (),
        //    Sym(s)   => (),
        //    Num(n)   => (),
        //};
        //(res, env)
        (Expr::Atom(Atom::from("1")), envi)
    }
}
