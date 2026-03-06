use std::{collections::HashMap, fmt::Debug, rc::Rc};

use crate::{
    expr::Expr,
    pattern::program::{Program, VarId},
};

#[derive(Clone)]
pub(super) enum EnvBinding<'s, A: Clone + PartialEq> {
    One(&'s Expr<A>),
    Many(Rc<Vec<&'s Expr<A>>>),
}

#[derive(Debug, Clone)]
pub struct Environment<'p, 's, A: Clone + PartialEq> {
    bindings: HashMap<VarId, EnvBinding<'s, A>>,
    program: &'p Program<A>,
}

pub struct ErrorBindCollision;

impl<'p, 's, A: Clone + PartialEq> Environment<'p, 's, A> {
    pub(super) fn new(program: &'p Program<A>) -> Self {
        Self {
            bindings: HashMap::new(),
            program,
        }
    }

    pub(super) fn bind_one(
        &mut self,
        bind_var: VarId,
        subject: &'s Expr<A>,
    ) -> Result<bool, ErrorBindCollision> {
        match self.bindings.get(&bind_var) {
            Some(EnvBinding::One(bound_subject)) => {
                if subject == *bound_subject {
                    Ok(false)
                } else {
                    Err(ErrorBindCollision)
                }
            }
            None => {
                self.bindings.insert(bind_var, EnvBinding::One(subject));
                Ok(true)
            }
            _ => Err(ErrorBindCollision),
        }
    }

    pub(super) fn bind_seq(
        &mut self,
        bind_var: VarId,
        subjects: Rc<Vec<&'s Expr<A>>>,
    ) -> Result<bool, ErrorBindCollision> {
        match self.bindings.get(&bind_var) {
            Some(EnvBinding::Many(bound_subjects)) => {
                if bound_subjects.len() != subjects.len() {
                    return Err(ErrorBindCollision);
                }

                let all_equal = bound_subjects
                    .iter()
                    .zip(subjects.iter())
                    .all(|(a, b)| *a == *b);

                if all_equal {
                    Ok(false)
                } else {
                    Err(ErrorBindCollision)
                }
            }
            None => {
                self.bindings.insert(bind_var, EnvBinding::Many(subjects));
                Ok(true)
            }
            _ => Err(ErrorBindCollision),
        }
    }

    pub(super) fn unbind(&mut self, bind_var: VarId) {
        self.bindings.remove(&bind_var);
    }

    fn var_id_from_name<T: AsRef<str>>(&self, name: T) -> Option<VarId> {
        self.program.var_ids.get(name.as_ref()).cloned()
    }

    pub fn get_one<T: AsRef<str>>(&self, name: T) -> Option<&'s Expr<A>> {
        use EnvBinding::*;

        let var_id = self.var_id_from_name(name.as_ref())?;

        match self.bindings.get(&var_id)? {
            One(val) => Some(val),
            Many(_) => None,
        }
    }

    pub fn get_seq<T: AsRef<str>>(&self, name: T) -> Option<&[&'s Expr<A>]> {
        use EnvBinding::*;

        let var_id = self.var_id_from_name(name.as_ref())?;

        match self.bindings.get(&var_id)? {
            One(_) => None,
            Many(val) => Some(val.as_slice()),
        }
    }
}

impl<'p, 's, A> Environment<'p, 's, A>
where
    A: PartialEq + Clone + Default,
{
    pub fn fill(&self, target_expr: Expr<A>) -> Expr<A> {
        match target_expr {
            Expr::Atom { .. } if target_expr.is_symbol() => {
                // In case of a symbol -> Replace with blanks
                let name = target_expr.get_symbol().unwrap();
                self.get_one(name).cloned().unwrap_or(target_expr)
            }
            Expr::Node { head, args, .. } => {
                let new_head = self.fill(*head);
                let mut new_args = vec![];

                for arg in args.into_iter() {
                    let Some(name) = arg.get_symbol() else {
                        // Arg is not a symbol -> decend and push to new args
                        new_args.push(self.fill(arg));
                        continue;
                    };

                    if let Some(repl) = self.get_one(name) {
                        new_args.push(repl.clone());
                    } else if let Some(repl) = self.get_seq(name) {
                        new_args.extend(repl.iter().map(|&e| e.clone()));
                    } else {
                        new_args.push(arg);
                    }
                }

                Expr::new_node(new_head, new_args)
            }
            _ => target_expr,
        }
    }
}

impl<'p, 's, A: Clone + PartialEq + Debug> Environment<'p, 's, A> {
    pub fn dbg_print_bindings(&self) {
        let mut keys: Vec<&VarId> = self.bindings.keys().collect();
        keys.sort();

        for k in keys {
            let v = self.bindings.get(k).unwrap();
            let name = self.program.vars.get(*k as usize).unwrap();
            eprintln!("{name}: {v:?}");
        }
    }
}
