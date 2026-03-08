mod util;

use common::{ClientMessage, KernelMessage};
use parser::parse;
use symbolics::expr::RawExpr;
use symbolics::format::MathDisplay;
use symbolics::simplify::Simplifier;
use wasm_bindgen::prelude::*;

use crate::util::escape_json;

#[wasm_bindgen]
pub fn eval_input(input: &str) -> String {
    let res = eval_inner(input.to_string());

    match res {
        Ok(msg) => serde_json::to_string(&msg).unwrap_or_else(|e| {
            format!(
                r#"{{"type":"SerializeError","msg":"{}"}}"#,
                escape_json(&e.to_string())
            )
        }),
        Err(msg) => serde_json::to_string(&msg).unwrap_or_else(|e| {
            format!(
                r#"{{"type":"SerializeError","msg":"{}"}}"#,
                escape_json(&e.to_string())
            )
        }),
    }
}

fn eval_inner(input: String) -> Result<KernelMessage, KernelMessage> {
    let input: ClientMessage =
        serde_json::from_str(&input).map_err(|err| KernelMessage::ParseError {
            input: "n/a".to_string(),
            msg: format!("Cannot unpack inbound message: {err}"),
        })?;

    let ClientMessage::Eval(input) = input;

    let ast_in = parse(&input).map_err(|err| KernelMessage::ParseError {
        input: input.clone(),
        msg: format!("Error parsing input: {}", err),
    })?;

    let input_expr = RawExpr::from(ast_in);
    let input_latex = input_expr.to_latex();
    let input_expr = input_expr.normalize();

    let result_expr = Simplifier::new(input_expr)
        .simple()
        .resugar()
        .canonicalize();

    Ok(KernelMessage::EvalResult {
        input: input_latex,
        output: result_expr.to_latex(),
    })
}
