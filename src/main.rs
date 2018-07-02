extern crate lalrpop_util;
#[macro_use] extern crate im;
extern crate rug;

mod ast;
mod cat_grammar;

use rug::Rational;
use ast::Expr::{self, *};

macro_rules! list {
    ( $( $x:expr ),* ) => { List(vec![$($x,)*]) };
    ( $( $x:expr ),* ,) => { List(vec![$($x,)*]) };
}

fn main() {
    let tree =
        list![
            list![
                atom("lambda"),
                list![atom("x")],
                list![
                    atom("+"),
                    atom("x"),
                    atom("x"),
                ],
            ],
            atom("22/7")
        ];

    println!("{:?}", tree);
    println!("{}", tree);
}

fn parse<'a>(src: &'a str) -> Expr {
    let parser = cat_grammar::ExprParser::new();
    parser.parse(src).unwrap()
}

fn atom<'a>(src: &'a str) -> Expr {
    Expr::atom(src)
}

#[test]
fn test_atom_conversion_fn() {
    assert_eq!(Expr::atom("123"),    Num(Rational::from((123, 1))));
    assert_eq!(Expr::atom("22/7"),   Num(Rational::from((22, 7))));
    assert_eq!(Expr::atom("lambda"), Sym("lambda".to_string()));
    assert_eq!(Expr::atom("#t"),     Bool(true));
    assert_eq!(Expr::atom("#f"),     Bool(false));
}

#[test]
fn test_atoms() {
    assert_eq!(parse("123"),    atom("123"));
    assert_eq!(parse("22/7"),   atom("22/7"));
    assert_eq!(parse("lambda"), atom("lambda"));
    assert_eq!(parse("#t"),     Bool(true));
    assert_eq!(parse("#f"),     Bool(false));
}

#[test]
fn test_lists() {
    let tree = list![atom("+"), atom("1"), atom("2")];
    assert_eq!(parse("(+ 1 2)"), tree); 

    let tree =
        list![
            atom("+"),
            atom("1"),
            list![
                atom("*"),
                atom("2"),
                atom("3"),
            ],
            atom("4"),
        ];
    assert_eq!(parse("(+ 1 (* 2 3) 4)"), tree);
}

