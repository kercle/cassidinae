use std::fmt::Debug;

use crate::{
    expr::{Expr, NormalizedExpr},
    matcher::CommutativePredicate,
    parser::ast::{ADD_HEAD, MUL_HEAD},
    pattern::{
        environment::Environment,
        program::{Compiler, Program},
        runtime::Runtime,
    },
};

pub type RuleTransformer<A> = Box<dyn Fn(&Environment<'_, '_, A>) -> Expr<A> + Send + Sync>;

pub struct Rule<A>
where
    A: Clone + PartialEq,
{
    // pub matcher: Matcher<A>,
    pub program: Program<A>,
    pub transform: RuleTransformer<A>,
}

#[derive(Default)]
pub struct Rewriter<A>
where
    A: Clone + PartialEq,
{
    rules: Vec<Rule<A>>,
    is_commutative: Option<CommutativePredicate<A>>,
}

impl<A> Rewriter<A>
where
    A: Clone + PartialEq + Default,
{
    pub fn new() -> Self {
        Self::default()
    }
}

impl<A> Rewriter<A>
where
    A: Clone + PartialEq + Default + Debug,
{
    pub fn commutative_if<F>(mut self, f: F) -> Self
    where
        F: Fn(&Expr<A>) -> bool + 'static,
    {
        debug_assert!(
            self.rules.is_empty(),
            "call commutative_if() before adding rules"
        );
        self.is_commutative = Some(CommutativePredicate::new(f));
        self
    }

    pub fn with_rule<F>(mut self, pattern: NormalizedExpr<A>, transform: F) -> Self
    where
        F: Fn(&Environment<'_, '_, A>) -> Expr<A> + Send + Sync + 'static,
    {
        // let matcher = Matcher::new(pattern.take_expr())
        //     .with_commutative_predicate(self.is_commutative.clone());
        // let program = Compiler::default().compile(&pattern.take_expr());

        self.rules.push(Rule {
            program: Compiler::default().compile(&pattern.take_expr()),
            transform: Box::new(transform),
        });
        self
    }

    pub fn with_rules<I, F>(mut self, rules: I) -> Self
    where
        I: IntoIterator<Item = (NormalizedExpr<A>, F)>,
        F: Fn(&Environment<'_, '_, A>) -> Expr<A> + Send + Sync + 'static,
    {
        for (p, t) in rules {
            self = self.with_rule(p, t);
        }
        self
    }

    pub fn apply_first_match(&self, expr: NormalizedExpr<A>) -> NormalizedExpr<A> {
        let res = expr.take_expr().map_bottom_up(&|expr| {
            let mut sub_expr = expr;

            for rule in &self.rules {
                let mut runtime = Runtime::new(&rule.program, &sub_expr);
                if let Some(mut env) = runtime.first_match() {
                    let f = &rule.transform;
                    sub_expr = f(&mut env).normalize();
                    break;
                }
            }

            sub_expr
        });

        NormalizedExpr::new(res)
    }
}

impl<A> Expr<A>
where
    A: Default + Clone + PartialEq + Debug,
{
    pub fn apply_until_fixed_point<F, I>(self, rules: I, limit_guard: u32) -> NormalizedExpr<A>
    where
        I: IntoIterator<Item = (NormalizedExpr<A>, F)>,
        F: Fn(&Environment<'_, '_, A>) -> Expr<A> + Send + Sync + 'static,
    {
        let rw: Rewriter<A> = Rewriter::new()
            .commutative_if(|head| head.matches_symbol(ADD_HEAD) || head.matches_symbol(MUL_HEAD))
            .with_rules(rules);

        let mut expr = NormalizedExpr::new(self);

        for _ in 0..limit_guard {
            let expr_next_iter = rw.apply_first_match(expr.clone());

            if expr != expr_next_iter {
                expr = expr_next_iter;
            } else {
                return expr;
            }
        }

        expr
    }
}
