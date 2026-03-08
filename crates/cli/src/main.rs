use clap::Parser;
use parser::parse;
use symbolics::expr::RawExpr;
use symbolics::format::MathDisplay;
use symbolics::simplify::Simplifier;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    input: String,

    #[arg(short, long, default_value = "latex")]
    output_type: String,
}

fn print_markdown(input: &str) {
    let ast = parse(input).unwrap_or_else(|err| {
        eprintln!("Error parsing input: {}", err);
        std::process::exit(1);
    });

    let expr = RawExpr::from(ast).normalize();
    let result = Simplifier::new(expr).simple().resugar().canonicalize();

    let latex = result.to_latex();
    println!("$$\n{}\n$$", latex);
}

fn main() {
    let args = Args::parse();

    match args.output_type.as_str() {
        "latex" => {
            print_markdown(&args.input);
        }
        _ => {
            eprintln!("Unsupported output type: {}", args.output_type);
            std::process::exit(1);
        }
    };
}
