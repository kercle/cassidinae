# A toy CAS in Rust

This project emerged from my PDE solver where I experimented with WebGL and WebAssembly compiled from Rust. I took the basic implementation of the parser I implemented and made it a bit more robust. Furthermore, I added a (pretty naive) `BigInteger` and `Rational` implementation. For the AST, I implementated a first draft of a cannonicalization procedure, with constant folding and gathering terms with constant coefficients. From the AST, Latex output can be generated.

## Design goals

Since this is just a toy project of mine, I want to keep the core modules independent of external crates. For now the CLI tool links against the library. In the future, I'd like to have a dedicated kernel that a UI application can connect to.

## Goals

- [ ] Decouple `AstNode` from internal `Expr`
- [ ] Pattern matching and rewrite engine
- [ ] Basic polynomial engine
- [ ] Naive Differentiation and Integration
- [ ] Matrix/Vector operations
- [ ] Plotting and dynamical variables for dashboards
- [ ] Numerical integration and ODE/PDE solving
- [ ] Numerical solver for equations
- [ ] UI maybe with Tauri
