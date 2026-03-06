use proc_macro2::TokenStream;
use quote::quote;

#[proc_macro]
pub fn expr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_string = token_stream_to_string(input.into());
    // let parser_ast = 

    match parser::parse(&input_string) {
        Ok(ast) => quote! { ... }.into(),
        Err(e) => {
            let msg = e.to_string();
            quote! { compile_error!(#msg) }.into()
        }
    }
}

fn token_stream_to_string(input: TokenStream) -> String {
    let mut result = String::new();
    let mut prev_end: Option<proc_macro2::LineColumn> = None;

    for tt in input {
        let span = tt.span();
        let start = span.start();

        let adjacent = prev_end
            .map(|end| end.line == start.line && end.column == start.column)
            .unwrap_or(false);

        if !adjacent && !result.is_empty() {
            result.push(' ');
        }

        result.push_str(&tt.to_string());
        prev_end = Some(span.end());
    }

    result
}
