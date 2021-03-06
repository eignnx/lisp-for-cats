extern crate im;

mod procedure;
use self::procedure::*;

use std::fmt;

use im::hashmap::HashMap;
use rug::Rational;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Expr {
    Sym(String),
    Num(Rational),
    Bool(bool),
    List(Vec<Expr>),
    Proc(Procedure),
}

use Expr::*;

pub type Context = HashMap<String, Expr>;

impl Expr {

    pub fn eval(&self, ctx: Context) -> (Expr, Context) {
        (self.clone(), ctx)
    }

    pub fn atom<'a>(src: &'a str) -> Self {
        if let Ok(n) = src.parse::<Rational>() {
            Expr::Num(n)
        } else {
            match src {
                "#t" => Expr::Bool(true),
                "#f" => Expr::Bool(false),
                _    => Expr::Sym(src.to_string()),
            }
        }
    }

    pub fn list(v: Vec<Expr>) -> Self {
        Expr::List(v)
    }

    fn tree_print(&self, f: &mut fmt::Formatter, level: usize)
        -> fmt::Result
    {
        const TAB: &'static str = "    ";
        let indent = TAB.repeat(level);
        match self {
            Sym(s)  => writeln!(f, "{}(Sym {})",  indent, s),
            Num(r)  => writeln!(f, "{}(Num {})",  indent, r),
            Bool(b) => writeln!(f, "{}(Bool {})", indent, b),
            Proc(p) => writeln!(f, "{}(Proc {})", indent, p.id()),
            List(v) => {
                writeln!(f, "{}(List", indent)?;
                for child in v {
                    child.tree_print(f, level + 1)?;
                }
                writeln!(f, "{})", indent)
            },
        }
    }
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.tree_print(f, 0)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Sym(s)  => write!(f, "{}", s),
            Num(r)  => write!(f, "{}", r),
            Bool(b) => write!(f, "{}", b),
            Proc(p) => write!(f, "{}", p),
            List(v) => match v.split_last() {
                Some((last, but_last)) => {
                    write!(f, "(")?;
                    for child in but_last {
                        child.fmt(f)?;
                        write!(f, " ")?;
                    }
                    last.fmt(f)?;
                    write!(f, ")")
                },
                None => unimplemented!("Are empty s-exprs ok?"),
            },
        }
    }
}

