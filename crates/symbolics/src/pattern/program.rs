use std::collections::HashMap;
use std::fmt::Debug;

use crate::expr::Expr;
use crate::pattern::{PATTERN_HEAD, builtin::*};

pub type InstrId = usize;
pub type VarId = u32;

pub struct Program<A: Clone + PartialEq> {
    pub entry: InstrId,
    pub instructions: Vec<Instruction<A>>,
    pub vars: Vec<String>,
}

#[derive(Debug)]
pub enum Quantity {
    One,
    Many { min: usize },
}

#[derive(Debug)]
pub enum Predicate {
    IsNumberQ,
    IsSymbolQ,
}

#[derive(Debug)]
pub enum Instruction<A: Clone + PartialEq> {
    Literal(Expr<A>),
    Variadic {
        quantity: Quantity,
        head_pattern: Option<InstrId>,
    },
    Bind {
        variable: VarId,
        inner: InstrId,
    },
    Predicate {
        predicate: Predicate,
        inner: InstrId,
    },
    Node {
        head: InstrId,
        plan: ArgPlan<A>,
    },
}

#[derive(Debug)]
pub enum ArgPlan<A: Clone + PartialEq> {
    Sequence(Vec<InstrId>),
    Multiset(MultisetPlan<A>),
}

#[derive(Debug)]
enum ArgOrder {
    Sequence,
    _Multiset,
}

#[derive(Debug)]
pub struct MultisetPlan<A: Clone + PartialEq> {
    pub literals: Vec<Expr<A>>,
    pub fixed: Vec<InstrId>,
    pub rest: Vec<(VarId, usize)>,
}

pub struct Compiler<A: Clone + PartialEq> {
    instructions: Vec<Instruction<A>>,
    var_ids: HashMap<String, VarId>,
    vars: Vec<String>,
}

impl<A: Clone + PartialEq + Default> Compiler<A> {
    pub fn new() -> Self {
        Compiler {
            instructions: Vec::new(),
            var_ids: HashMap::new(),
            vars: Vec::new(),
        }
    }

    fn emit(&mut self, instr: Instruction<A>) -> InstrId {
        let id = self.instructions.len();
        self.instructions.push(instr);
        id
    }

    fn bind_name_id(&mut self, name: &str) -> VarId {
        if let Some(&id) = self.var_ids.get(name) {
            return id;
        }
        let id = self.vars.len() as VarId;
        self.vars.push(name.to_string());
        self.var_ids.insert(name.to_string(), id);
        id
    }

    pub fn compile(mut self, pat: &Expr<A>) -> Program<A> {
        let entry = self.compile_pat(pat);

        Program {
            entry,
            instructions: self.instructions,
            vars: self.vars,
        }
    }

    fn compile_pat(&mut self, pat_expr: &Expr<A>) -> InstrId {
        use Expr::*;
        match pat_expr {
            Atom { .. } => self.emit(Instruction::Literal(pat_expr.clone())),
            Node { head, args, .. } if head.matches_symbol(HEAD_BLANK) && args.len() <= 1 => {
                self.compile_blank_with_head_constraint(Quantity::One, args.first())
            }
            Node { head, args, .. }
                if head.matches_symbol(HEAD_BLANK_SEQUENCE) && args.len() <= 1 =>
            {
                self.compile_blank_with_head_constraint(Quantity::Many { min: 1 }, args.first())
            }
            Node { head, args, .. }
                if head.matches_symbol(HEAD_BLANK_NULL_SEQUENCE) && args.len() <= 1 =>
            {
                self.compile_blank_with_head_constraint(Quantity::Many { min: 0 }, args.first())
            }
            Node { head, args, .. } if pat_expr.is_application_of(PATTERN_HEAD, 2) => {
                let [lhs, rhs] = args.as_slice() else {
                    unreachable!()
                };

                let Some(bind_var_name) = lhs.get_symbol() else {
                    return self.compile_pat_node(head, ArgOrder::Sequence, &args);
                };

                let var_id = self.bind_name_id(bind_var_name);
                let inner = self.compile_pat(rhs);
                self.emit(Instruction::Bind {
                    variable: var_id,
                    inner,
                })
            }
            Node { head, args, .. } => self.compile_pat_node(head, ArgOrder::Sequence, &args),
        }
    }

    fn compile_blank_with_head_constraint(
        &mut self,
        quantity: Quantity,
        head_pattern: Option<&Expr<A>>,
    ) -> InstrId {
        let head_pattern = if let Some(e) = head_pattern {
            Some(self.compile_pat(e))
        } else {
            None
        };

        self.emit(Instruction::Variadic {
            quantity,
            head_pattern,
        })
    }

    fn compile_pat_node(
        &mut self,
        head: &Expr<A>,
        arg_order: ArgOrder,
        children: &[Expr<A>],
    ) -> InstrId {
        let head = Self::compile_pat(self, head);

        let plan = match arg_order {
            ArgOrder::Sequence => {
                let pats = children.iter().map(|c| self.compile_pat(c)).collect();
                ArgPlan::Sequence(pats)
            }
            ArgOrder::_Multiset => {
                let plan = self.compile_unordered(children);
                ArgPlan::Multiset(plan)
            }
        };

        self.emit(Instruction::Node { head, plan })
    }

    fn compile_unordered(&mut self, _children: &[Expr<A>]) -> MultisetPlan<A> {
        todo!()
        // let mut literals = Vec::new();
        // let mut fixed = Vec::new();
        // let mut rest: Vec<(VarId, usize)> = vec![];

        // for c in children {
        //     todo!()
        // }

        // if rest.len() > 1 {
        //     unimplemented!(
        //         "Matching unordered children with more than 1 variadic pattern not supported yet"
        //     )
        // }

        // MultisetPlan {
        //     literals,
        //     fixed,
        //     rest,
        // }
    }
}
