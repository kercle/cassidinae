use crate::{
    builtin::*,
    expr::{ExprKind, NormExpr, RawExpr},
};
use numbers::Number;

use crate::builtin::CANNONICAL_HEAD_SQRT;

impl NormExpr {
    fn resugar_add(mut args: Vec<RawExpr>) -> RawExpr {
        args.reverse();

        let mut new_args = Vec::with_capacity(args.len());

        let (coeff, term) = NormExpr::split_coefficient(args.pop().unwrap());
        if coeff.is_one() {
            new_args.push(term);
        } else if coeff.is_minus_one() {
            new_args.push(RawExpr::new_node(NEG_HEAD, vec![term]));
        } else if coeff.is_zero() {
            // In normalized expression, this should not happen
            unreachable!()
        } else {
            new_args.push(RawExpr::collapse_mul(vec![coeff.into(), term]));
        }

        while let Some(arg) = args.pop() {
            let (coeff, term) = NormExpr::split_coefficient(arg);

            if coeff.is_one() {
                new_args.push(term);
            } else if coeff.is_minus_one() {
                let lhs = new_args.pop().unwrap();
                new_args.push(RawExpr::new_node(SUB_HEAD, vec![lhs, term]));
            } else if coeff.is_negative() {
                let lhs = new_args.pop().unwrap();
                new_args.push(RawExpr::new_node(
                    SUB_HEAD,
                    vec![lhs, RawExpr::collapse_mul(vec![coeff.abs().into(), term])],
                ));
            } else if coeff.is_zero() {
                // In normalized expression, this should not happen
                unreachable!()
            } else {
                new_args.push(RawExpr::collapse_mul(vec![coeff.into(), term]));
            }
        }

        if new_args.len() == 1 {
            new_args.pop().unwrap()
        } else {
            RawExpr::collapse_add(new_args)
        }
    }

    fn resugar_mul(args: Vec<RawExpr>) -> RawExpr {
        let mut numerator = Vec::with_capacity(args.len());
        let mut denominator = Vec::with_capacity(args.len());

        for a in args.into_iter() {
            if let Some((lhs, rhs)) = a.unpack_binary_node(POW_HEAD) {
                let (mut coeff, rhs_rest) = NormExpr::split_coefficient(rhs.clone());
                if coeff.is_negative() {
                    coeff.flip_sign();
                    denominator.push(RawExpr::new_node(
                        POW_HEAD,
                        vec![
                            lhs.clone(),
                            RawExpr::collapse_mul(vec![coeff.into(), rhs_rest]),
                        ],
                    ));
                } else {
                    numerator.push(a);
                }
            } else if let Some((lhs, rhs)) = a.unpack_binary_node(DIV_HEAD) {
                numerator.push(lhs.clone());
                denominator.push(rhs.clone());
            } else {
                numerator.push(a);
            }
        }

        if denominator.is_empty() {
            RawExpr::collapse_mul(numerator)
        } else if numerator.is_empty() {
            RawExpr::new_node(
                DIV_HEAD,
                vec![
                    RawExpr::new_number(Number::one()),
                    RawExpr::collapse_mul(denominator),
                ],
            )
        } else {
            let lhs = if numerator.len() >= 2 {
                RawExpr::collapse_mul(numerator)
            } else {
                numerator.pop().unwrap()
            };

            let rhs = if denominator.len() >= 2 {
                RawExpr::collapse_mul(denominator)
            } else {
                denominator.pop().unwrap()
            };

            RawExpr::new_node(DIV_HEAD, vec![lhs, rhs])
        }
    }

    pub fn resugar(self) -> RawExpr {
        match self.kind {
            ExprKind::Node { head, args, .. }
                if head.matches_symbol(ADD_HEAD) && !args.is_empty() =>
            {
                let args = args.into_iter().map(|e| e.resugar()).collect();
                Self::resugar_add(args)
            }
            ExprKind::Node { head, args, .. }
                if head.matches_symbol(MUL_HEAD) && !args.is_empty() =>
            {
                let args = args.into_iter().map(|e| e.resugar()).collect();

                Self::resugar_mul(args)
            }
            ExprKind::Node { head, args, .. }
                if head.matches_symbol(POW_HEAD) && args.len() == 2 =>
            {
                let one_half = Number::new_rational_from_i64(1, 2).unwrap();
                if args
                    .last()
                    .unwrap()
                    .get_number()
                    .map(|e| e == &one_half)
                    .unwrap_or(false)
                {
                    return RawExpr::new_node(
                        CANNONICAL_HEAD_SQRT,
                        vec![args.first().unwrap().clone().into_raw()],
                    );
                }

                let args = args.into_iter().map(|e| e.resugar()).collect();

                Self::resugar_mul(vec![RawExpr::new_node(head.into_raw(), args)])
            }
            _ => self.into_raw(),
        }
    }
}
