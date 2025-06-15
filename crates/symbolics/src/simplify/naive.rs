use numbers::{RealScalar, integer::BigInteger, rational::Rational};

use crate::parser::ast::AstNode;

fn fold_constants(node: AstNode) -> AstNode {
    use AstNode::*;
    match &node {
        Add(lhs, rhs) => {
            return fold_constants(AddSeq(vec![*lhs.to_owned(), *rhs.to_owned()]));
        }
        Sub(lhs, rhs) => {
            if let (Constant { value: l, .. }, Constant { value: r, .. }) =
                (lhs.as_ref(), rhs.as_ref())
            {
                return (l - r).map_or_else(|| node, |value| AstNode::constant(value));
            }
        }
        Mul(lhs, rhs) => {
            return fold_constants(MulSeq(vec![*lhs.to_owned(), *rhs.to_owned()]));
        }
        Div(lhs, rhs) => {
            if let (
                Constant {
                    value: RealScalar::Integer(l),
                    ..
                },
                Constant {
                    value: RealScalar::Integer(r),
                    ..
                },
            ) = (lhs.as_ref(), rhs.as_ref())
            {
                if r.is_zero() {
                    todo!("Handle division by zero");
                }
                let rational = Rational::new(l.clone().into(), r.clone().into())
                    .expect("todo: handle invalid rational");

                return AstNode::constant(RealScalar::Rational(rational));
            }
        }
        AddSeq(nodes) => {
            let mut sum = RealScalar::zero();
            let mut new_nodes = vec![];

            for node in nodes.iter() {
                if let Constant { value, .. } = node {
                    sum = value + &sum;
                } else {
                    new_nodes.push(node.clone());
                }
            }

            if !sum.is_zero() || new_nodes.is_empty() {
                new_nodes.insert(0, AstNode::constant(sum.clone()));
            }

            if new_nodes.len() == 1 {
                return new_nodes.pop().unwrap();
            } else {
                return AddSeq(new_nodes);
            }
        }
        MulSeq(nodes) => {
            let mut product = RealScalar::one();
            let mut new_nodes = vec![];

            for node in nodes.iter() {
                if let Constant { value, .. } = node {
                    if let Some(value) = value * &product {
                        product = value;
                    } else {
                        new_nodes.push(node.clone());
                    }
                } else {
                    new_nodes.push(node.clone());
                }

                if product.is_zero() {
                    return AstNode::constant(RealScalar::zero());
                }
            }

            if !product.is_one() || new_nodes.is_empty() {
                new_nodes.insert(0, AstNode::constant(product.clone()));
            }

            if new_nodes.len() == 1 {
                return new_nodes.pop().unwrap();
            } else {
                return MulSeq(new_nodes);
            }
        }
        Pow(lhs, rhs) => {
            if let (
                Constant {
                    value: RealScalar::Integer(base),
                    ..
                },
                Constant {
                    value: RealScalar::Integer(exp),
                    ..
                },
            ) = (lhs.as_ref(), rhs.as_ref())
            {
                if exp.is_zero() {
                    return AstNode::constant(RealScalar::one());
                } else if exp.is_one() {
                    return AstNode::constant(RealScalar::Integer(base.clone()));
                }

                let abs_exp = exp.abs();
                let result = base.pow(abs_exp.abs());

                if let Ok(result) = result {
                    if exp.is_positive() {
                        return AstNode::constant(RealScalar::Integer(result));
                    }

                    return AstNode::constant(RealScalar::Rational(
                        Rational::new(BigInteger::one(), result)
                            .expect("todo: handle invalid rational"),
                    ));
                }
            }
        }
        Negation(node) => {
            if let Constant { value, .. } = node.as_ref() {
                return AstNode::constant(-value.clone());
            }
        }
        _ => {}
    };

    node
}

fn gather_common_terms(node: AstNode) -> AstNode {
    use AstNode::*;

    fn split_multiple_of_constant(node: AstNode) -> (RealScalar, AstNode) {
        if let MulSeq(nodes) = &node {
            if let Some((constant, rest)) = nodes.split_first() {
                if let Constant { value, .. } = constant {
                    return (value.clone(), flatten_commutative(MulSeq(rest.to_vec())));
                }
            }
        } else if let Mul(lhs, rhs) = &node {
            return split_multiple_of_constant(MulSeq(vec![*lhs.to_owned(), *rhs.to_owned()]));
        }
        (RealScalar::one(), node)
    }

    match &node {
        AddSeq(nodes) => {
            let mut terms_with_factors: Vec<(RealScalar, AstNode)> = vec![];

            for node in nodes.iter() {
                let (factor, term) = split_multiple_of_constant(node.clone());

                let mut found = false;
                for (existing_factor, existing_term) in terms_with_factors.iter_mut() {
                    if *existing_term == term {
                        // let existing_factor = existing_factor.clone();
                        *existing_factor = &*existing_factor + &factor;
                        found = true;
                        break;
                    }
                }

                if !found {
                    terms_with_factors.push((factor, term));
                }
            }

            return AddSeq(
                terms_with_factors
                    .into_iter()
                    .map(|(factor, term)| {
                        if factor.is_one() {
                            term
                        } else {
                            MulSeq(vec![AstNode::constant(factor), term])
                        }
                    })
                    .collect(),
            );
        }
        _ => {}
    }

    node
}

fn expand_subtraction(node: AstNode) -> AstNode {
    use AstNode::*;
    match &node {
        Sub(lhs, rhs) => {
            let lhs = Box::new(expand_subtraction(*lhs.to_owned()));
            let rhs = Box::new(expand_subtraction(*rhs.to_owned()));

            return Add(lhs, Box::new(Negation(rhs)));
        }
        _ => {}
    }

    node
}

fn flatten_commutative(node: AstNode) -> AstNode {
    use AstNode::*;

    fn flatten_commutative_inner<F>(nodes: &[AstNode], extract_func: F) -> Vec<AstNode>
    where
        F: Fn(&AstNode) -> Option<Vec<AstNode>>,
    {
        let mut flattened_nodes = vec![];
        for node in nodes.iter() {
            let node = flatten_commutative(node.clone());

            if let Some(mut inner_nodes) = extract_func(&node) {
                flattened_nodes.append(&mut inner_nodes);
            } else {
                flattened_nodes.push(node);
            }
        }

        flattened_nodes
    }

    match &node {
        Add(lhs, rhs) => {
            return flatten_commutative(AddSeq(vec![*lhs.to_owned(), *rhs.to_owned()]));
        }
        AddSeq(nodes) => {
            let mut flattened_nodes = flatten_commutative_inner(nodes, |node| {
                if let AddSeq(inner_nodes) = node {
                    Some(inner_nodes.clone())
                } else {
                    None
                }
            });

            if flattened_nodes.is_empty() {
                return AstNode::constant(RealScalar::zero());
            } else if flattened_nodes.len() == 1 {
                return flattened_nodes.pop().unwrap();
            } else {
                return AddSeq(flattened_nodes);
            }
        }
        Negation(node) => {
            return flatten_commutative(MulSeq(vec![
                AstNode::constant(RealScalar::minus_one()),
                *node.to_owned(),
            ]));
        }
        Mul(lhs, rhs) => {
            return flatten_commutative(MulSeq(vec![*lhs.to_owned(), *rhs.to_owned()]));
        }
        MulSeq(nodes) => {
            let mut flattened_nodes = flatten_commutative_inner(nodes, |node| {
                if let MulSeq(inner_nodes) = node {
                    Some(inner_nodes.clone())
                } else {
                    None
                }
            });

            if flattened_nodes.is_empty() {
                return AstNode::constant(RealScalar::one());
            } else if flattened_nodes.len() == 1 {
                return flattened_nodes.pop().unwrap();
            } else {
                return MulSeq(flattened_nodes);
            }
        }
        _ => {}
    }

    node
}

fn cannonical_order(node: AstNode) -> AstNode {
    use AstNode::*;

    match &node {
        AddSeq(nodes) => {
            let mut sorted_nodes = nodes.clone();
            sorted_nodes.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            return AddSeq(sorted_nodes);
        }
        MulSeq(nodes) => {
            let mut sorted_nodes = nodes.clone();
            sorted_nodes.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            return MulSeq(sorted_nodes);
        }
        _ => {}
    }

    node
}

fn unflatten_commutative(node: AstNode) -> AstNode {
    use AstNode::*;

    let altered_node = match &node {
        AddSeq(nodes) => Some(if nodes.len() == 1 {
            nodes[0].clone()
        } else if nodes.len() == 2 {
            Add(Box::new(nodes[0].clone()), Box::new(nodes[1].clone()))
        } else {
            AddSeq(vec![
                Add(Box::new(nodes[0].clone()), Box::new(nodes[1].clone())),
                AddSeq(nodes[2..].to_vec()),
            ])
        }),
        MulSeq(nodes) => Some(if nodes.len() == 1 {
            nodes[0].clone()
        } else if nodes.len() == 2 {
            Mul(Box::new(nodes[0].clone()), Box::new(nodes[1].clone()))
        } else {
            MulSeq(vec![
                Mul(Box::new(nodes[0].clone()), Box::new(nodes[1].clone())),
                MulSeq(nodes[2..].to_vec()),
            ])
        }),
        _ => None,
    };

    altered_node.map_or_else(|| node, |n| n.map(unflatten_commutative))
}

fn simplify_add_neg_to_sub(node: AstNode) -> AstNode {
    // Maybe this can be generalized in some factorization step?

    use AstNode::*;
    match &node {
        Add(lhs, rhs) => {
            let lhs = simplify_add_neg_to_sub(*lhs.clone());
            let rhs = simplify_add_neg_to_sub(*rhs.clone());

            if let (Negation(neg_lhs), Negation(neg_rhs)) = (&lhs, &rhs) {
                return Negation(Box::new(Add(neg_lhs.to_owned(), neg_rhs.to_owned())));
            } else if let Negation(neg_rhs) = rhs {
                return Sub(Box::new(lhs), neg_rhs);
            } else if let Negation(neg_lhs) = lhs {
                return Sub(Box::new(rhs), neg_lhs);
            }
        }
        Mul(lhs, rhs) => {
            if let Constant { value, .. } = *lhs.clone() {
                if value < RealScalar::zero() {
                    return Negation(Box::new(Mul(
                        Box::new(AstNode::constant(-value.clone())),
                        Box::new(simplify_add_neg_to_sub(*rhs.to_owned())),
                    )));
                }
            }
        }
        _ => {}
    }

    node
}

pub fn simplify_ast(mut tree: AstNode) -> AstNode {
    loop {
        let tree_iteration = tree
            .clone()
            .map(expand_subtraction)
            .map(flatten_commutative)
            .map(fold_constants)
            .map(gather_common_terms)
            .map(cannonical_order)
            .map(unflatten_commutative)
            .map(simplify_add_neg_to_sub);
        if tree_iteration == tree {
            break tree_iteration;
        }
        tree = tree_iteration;
    }
}
