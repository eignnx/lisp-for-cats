extern crate lalrpop_util;
#[macro_use] extern crate im;

mod ast;
mod cat_grammar;

use ast::{Expr::*, Atom::*};

#[test]
fn atom_recognition() {
    let atom_parser = cat_grammar::AtomTParser::new();

    let expected = atom_parser.parse("5").unwrap();
    let returned = Num(5);
    assert_eq!(returned, expected);

    let expected = atom_parser.parse("qwerty").unwrap();
    let returned = Sym("qwerty");
    assert_eq!(returned, expected);
}

#[test]
fn simple_expr_recognition() {
    let expr_parser = cat_grammar::ExprTParser::new();

    let returned = expr_parser.parse("5").unwrap();
    let expected = Atom(Num(5));
    assert_eq!(returned, expected);

    let returned = expr_parser.parse("qwerty").unwrap();
    let expected = Atom(Sym("qwerty"));
    assert_eq!(returned, expected);
}

#[test]
fn sexpr_recognition() {
    let expr_parser = cat_grammar::ExprTParser::new();

    let returned = expr_parser.parse("(+ 1 2)").unwrap();
    let expected =
        SExpr(conslist![
            Atom(Sym("+")),
            Atom(Num(1)),
            Atom(Num(2))
        ]);
    assert_eq!(returned, expected);
}


fn try_args(input: &str) {
    println!("Trying '{}'...", input);

    let expr_parser = cat_grammar::ExprTParser::new();
    let returned = expr_parser.parse(input);
    match returned {
        Ok(v) => println!("Success! got: {:?}", v),
        Err(e) => println!("\nError! got:\n{}\n\n", e),
    };
}

fn main() {
    try_args("(100 200 300)");
}



