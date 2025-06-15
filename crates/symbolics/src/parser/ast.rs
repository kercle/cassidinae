use std::cmp::{self, Ordering};

use crate::format::MathDisplay;
use numbers::RealScalar;

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode<Annotation = ()>
where
    Annotation: Clone,
{
    Constant {
        value: RealScalar,
        annotation: Annotation,
    },
    NamedValue {
        name: String,
        annotation: Annotation,
    },
    Add(Box<AstNode>, Box<AstNode>),
    AddSeq(Vec<AstNode>),
    Negation(Box<AstNode>),
    Sub(Box<AstNode>, Box<AstNode>),
    Mul(Box<AstNode>, Box<AstNode>),
    MulSeq(Vec<AstNode>),
    Reciprocal(Box<AstNode>),
    Div(Box<AstNode>, Box<AstNode>),
    Pow(Box<AstNode>, Box<AstNode>),
    Sin(Box<AstNode>),
    Cos(Box<AstNode>),
    Tan(Box<AstNode>),
    Sqrt(Box<AstNode>),
    FunctionCall {
        name: String,
        args: Vec<AstNode>,
    },
    Block(Vec<AstNode>),
}

impl<Annotation> AstNode<Annotation>
where
    Annotation: Default + Clone,
{
    pub fn constant(value: RealScalar) -> Self {
        AstNode::Constant {
            annotation: Annotation::default(),
            value,
        }
    }

    pub fn named_value(name: String) -> Self {
        AstNode::NamedValue {
            annotation: Annotation::default(),
            name,
        }
    }
}

impl AstNode {
    pub fn from_function_call(name: String, mut args: Vec<AstNode>) -> Result<Self, String> {
        let initial_args_len = args.len();

        let result = match name.as_str() {
            "sin" => Ok(AstNode::Sin(Box::new(
                args.pop().ok_or("sin requires one argument")?,
            ))),
            "cos" => Ok(AstNode::Cos(Box::new(
                args.pop().ok_or("cos requires one argument")?,
            ))),
            "tan" => Ok(AstNode::Tan(Box::new(
                args.pop().ok_or("tan requires one argument")?,
            ))),
            "sqrt" => Ok(AstNode::Sqrt(Box::new(
                args.pop().ok_or("sqrt requires one argument")?,
            ))),
            _ => {
                return Ok(AstNode::FunctionCall {
                    name: name.clone(),
                    args: args,
                });
            }
        };

        if !args.is_empty() {
            let expected_arg_count = initial_args_len - args.len();

            let arguments = if expected_arg_count == 1 {
                "argument"
            } else {
                "arguments"
            };

            return Err(format!(
                "Function {} takes {} {arguments} but {} given.",
                name,
                initial_args_len - args.len(),
                initial_args_len
            ));
        }

        result
    }

    pub fn map<F>(self, mut f: F) -> AstNode
    where
        F: FnMut(AstNode) -> AstNode,
    {
        self.map_inner(&mut f)
    }

    fn map_inner<F>(self, f: &mut F) -> AstNode
    where
        F: FnMut(AstNode) -> AstNode,
    {
        use AstNode::*;
        let mapped = match self {
            Add(l, r) => Add(Box::new(l.map_inner(f)), Box::new(r.map_inner(f))),
            AddSeq(nodes) => AddSeq(nodes.into_iter().map(|n| n.map_inner(f)).collect()),
            Negation(x) => Negation(Box::new(x.map_inner(f))),
            Sub(l, r) => Sub(Box::new(l.map_inner(f)), Box::new(r.map_inner(f))),
            Mul(l, r) => Mul(Box::new(l.map_inner(f)), Box::new(r.map_inner(f))),
            MulSeq(nodes) => MulSeq(nodes.into_iter().map(|n| n.map_inner(f)).collect()),
            Reciprocal(x) => Reciprocal(Box::new(x.map_inner(f))),
            Div(l, r) => Div(Box::new(l.map_inner(f)), Box::new(r.map_inner(f))),
            Pow(l, r) => Pow(Box::new(l.map_inner(f)), Box::new(r.map_inner(f))),
            Sin(x) => Sin(Box::new(x.map_inner(f))),
            Cos(x) => Cos(Box::new(x.map_inner(f))),
            Tan(x) => Tan(Box::new(x.map_inner(f))),
            Sqrt(x) => Sqrt(Box::new(x.map_inner(f))),
            FunctionCall { name, args } => FunctionCall {
                name,
                args: args.into_iter().map(|a| a.map_inner(f)).collect(),
            },
            Block(nodes) => Block(nodes.into_iter().map(|n| n.map_inner(f)).collect()),
            Constant { .. } | NamedValue { .. } => self,
        };
        f(mapped)
    }

    pub fn iter(&self) -> AstNodeIter {
        AstNodeIter::new(self)
    }

    pub fn is_constant(&self) -> bool {
        matches!(self, AstNode::Constant { .. })
    }
}

impl cmp::PartialOrd for AstNode {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        use AstNode::*;
        match (self, other) {
            (Constant { value: a, .. }, Constant { value: b, .. }) => a.partial_cmp(b),
            (Constant { .. }, _) => Some(Ordering::Less),
            (_, Constant { .. }) => Some(Ordering::Greater),
            (a, b) => a.to_yasc().partial_cmp(&b.to_yasc()),
        }
    }
}

pub struct AstNodeIter<'a> {
    stack: Vec<&'a AstNode>,
}

impl<'a> AstNodeIter<'a> {
    pub fn new(root: &'a AstNode) -> Self {
        AstNodeIter { stack: vec![root] }
    }
}

impl<'a> Iterator for AstNodeIter<'a> {
    type Item = &'a AstNode;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;

        use AstNode::*;
        match node {
            Negation(x) | Reciprocal(x) | Sin(x) | Cos(x) | Tan(x) | Sqrt(x) => {
                self.stack.push(x);
            }
            Add(lhs, rhs) | Sub(lhs, rhs) | Mul(lhs, rhs) | Div(lhs, rhs) | Pow(lhs, rhs) => {
                self.stack.push(rhs);
                self.stack.push(lhs);
            }
            AddSeq(args) | MulSeq(args) => {
                for arg in args.iter().rev() {
                    self.stack.push(arg);
                }
            }
            FunctionCall { args, .. } | Block(args) => {
                for arg in args.iter().rev() {
                    self.stack.push(arg);
                }
            }
            Constant { .. } | NamedValue { .. } => {}
        }
        Some(node)
    }
}
