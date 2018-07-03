use ast::Expr::{self, *};

type FnType = String; //dyn Fn(Expr) -> Expr;

pub struct ProcTracker {
    id_counter: u64,
}

impl ProcTracker {
    pub fn new() -> Self {
        ProcTracker { id_counter: 0 }
    }

    pub fn make_proc(&mut self, f: FnType) -> Procedure {
        let proc = Procedure::new(f, self.id_counter);
        self.id_counter += 1;
        proc
    }
}

pub struct Procedure {
    proc: Box<FnType>,
    id: u64,
}

impl Procedure {
    pub fn new(proc: FnType, id: u64) -> Self {
        Procedure { proc: Box::new(proc), id }
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}

use std::fmt;

impl fmt::Display for Procedure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#<procedure:{}>", self.id)
    }
}

impl fmt::Debug for Procedure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Procedure{{id:{}}})", self.id)
    }
}

impl Clone for Procedure {
    fn clone(&self) -> Self {
        unreachable!("You promised you wouldn't clone a Proc!");
    }
}

impl PartialEq for Procedure {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Procedure {}

use std::hash::{Hash, Hasher};

impl Hash for Procedure {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}


