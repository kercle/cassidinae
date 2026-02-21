use numbers::Number;

use crate::{
    atom::Atom,
    expr::Expr,
    parser::ast::{ADD_HEAD, MUL_HEAD, POW_HEAD, ParserAst},
};

impl<A, T: Into<Atom>> From<T> for Expr<A>
where
    A: Default,
{
    fn from(x: T) -> Self {
        Expr::Atom {
            entry: x.into(),
            annotation: A::default(),
        }
    }
}

impl<A> Expr<A>
where
    A: Default + Clone + PartialEq,
{
    pub fn from_i64(value: i64) -> Self {
        Self::new_number(Number::from_i64(value))
    }

    pub fn from_parser_ast(parser_ast: &ParserAst<A>) -> Self {
        match parser_ast {
            ParserAst::Constant { value, annotation } => {
                Self::new_number(value.clone()).with_annotation(annotation.clone())
            }
            ParserAst::Symbol { name, annotation } => {
                Self::new_symbol(name.clone()).with_annotation(annotation.clone())
            }
            ParserAst::Add { nodes, annotation } => {
                let head = Self::new_symbol(ADD_HEAD);
                let args = nodes
                    .iter()
                    .map(|node| Self::from_parser_ast(node))
                    .collect();
                Self::new_compound(head, args).with_annotation(annotation.clone())
            }
            ParserAst::Sub {
                lhs,
                rhs,
                annotation,
            } => {
                let head = Self::new_symbol(ADD_HEAD);
                let lhs = Self::from_parser_ast(lhs.as_ref());
                let rhs = Self::from_parser_ast(rhs.as_ref());

                Self::new_compound(
                    head,
                    vec![
                        lhs,
                        Self::new_compound(
                            Self::new_symbol(MUL_HEAD),
                            vec![Self::new_number(Number::from_i64(-1)), rhs],
                        ),
                    ],
                )
                .with_annotation(annotation.clone())
            }
            ParserAst::Negation { arg, annotation } => {
                let arg = Self::from_parser_ast(arg.as_ref());
                Self::new_compound(
                    Self::new_symbol(MUL_HEAD),
                    vec![Self::new_number(Number::from_i64(-1)), arg],
                )
                .with_annotation(annotation.clone())
            }
            ParserAst::Mul { nodes, annotation } => {
                let head = Self::new_symbol(MUL_HEAD);
                let args = nodes
                    .iter()
                    .map(|node| Self::from_parser_ast(node))
                    .collect();
                Self::new_compound(head, args).with_annotation(annotation.clone())
            }
            ParserAst::Div {
                lhs,
                rhs,
                annotation,
            } => {
                let head = Self::new_symbol(MUL_HEAD);
                let lhs = Self::from_parser_ast(lhs.as_ref());
                let rhs = Self::from_parser_ast(rhs.as_ref());

                Self::new_compound(
                    head,
                    vec![
                        lhs,
                        Self::new_compound(
                            Self::new_symbol(POW_HEAD),
                            vec![rhs, Self::new_number(Number::from_i64(-1))],
                        ),
                    ],
                )
                .with_annotation(annotation.clone())
            }
            ParserAst::Pow {
                lhs,
                rhs,
                annotation,
            } => {
                let head = Self::new_symbol(POW_HEAD);
                let lhs = Self::from_parser_ast(lhs.as_ref());
                let rhs = Self::from_parser_ast(rhs.as_ref());

                Self::new_compound(head, vec![lhs, rhs]).with_annotation(annotation.clone())
            }
            ParserAst::FunctionCall {
                name,
                args,
                annotation,
            } => {
                let head = Self::new_symbol(name);
                let args = args
                    .iter()
                    .map(|node| Self::from_parser_ast(node))
                    .collect();

                Self::new_compound(head, args).with_annotation(annotation.clone())
            }
            ParserAst::Block { .. } => todo!(),
        }
    }
}
