# A toy CAS in Rust

This project emerged from my PDE solver where I experimented with WebGL and WebAssembly compiled from Rust. I took the basic implementation of the parser I implemented and made it a bit more robust. Furthermore, I added a (pretty naive) `BigInteger` and `Rational` implementation. For the AST, I implementated a first draft of a cannonicalization procedure, with constant folding and gathering terms with constant coefficients. From the AST, Latex output can be generated.

## Design goals

Since this is just a toy project of mine, I want to keep the core modules independent of external crates. For now the CLI tool links against the library. In the future, I'd like to have a dedicated kernel that a UI application can connect to.

## Goals

- [x] Decouple `AstNode` from internal `Expr`
- [x] Basic Pattern matching
- [x] More advanced Pattern matching (commutative expressions with at most one sequence)
- [ ] Allow more sequences in commutative patterns
- [ ] Support for more flexible predicate in pattern matching
- [x] Basic Rewrite engine
- [ ] Basic polynomial engine
- [ ] Solve polynomial equations
- [x] Naive Differentiation
- [x] Naive Integration with simple rules
- [ ] More advanced integration
- [ ] Matrix/Vector operations
- [ ] Plotting and dynamical variables for dashboards
- [ ] Numerical integration and ODE/PDE solving
- [ ] Numerical solver for equations
- [ ] UI maybe with Tauri
- [x] Web frontend built with Svelte
- [ ] Compile to Web-Assembly and bundle with frontend
