# Set Parser 
 Set Parser lets you declare, compare and do operations on sets. Useful for first-year CS students who only start Discrete Math to get a rough understanding of Set Theory.
 In this language, a set consists of integers that are separated by comma (not necessarily true in real Set Theory!).
 See [grammar](./parser/src/grammar.pest) for more details.
 See [crates.io](https://crates.io/crates/set_parser)
 You can:

### 1. Declare and define a set

```
let A = {1, 2, 4};
let C = {1..10};
let D = ∅; // this is an empty set (yes, comments are available too)
```

### 2. Perform operations on sets

```
let E = A \ C;
let F = A ∩ C;
```

or something more complicated:

```
let E = (A' \ C) △ B;
let F = (A ∩ B)' △ (A △ B)
```

Available operations are:

- `A'` - complement (universe set must be declared)
- `A ∪ B` - union
- `A ∩ B` - intersection
- `A \ B` - difference
- `A △ B` - symmetric difference

### 3. Set universe value (useful for complements)

```
let universe = {1, 2, 3, 4, 5}
```

Example of usage:

```
let universe = {1, 2, 3, 4, 5}
let A = {1, 2}
let B = A'
print B
```

will output {3, 4, 5}  

### 4. Print a set

```
print A
print A △ B
```

## How Parsing Works

What will be implemented:

- A lexer that converts the input string into tokens (identifiers, symbols like `∩`, `∪`, `\`, `△`, `'`, parentheses, and `∅`).
- A top-down operator precedence parser that:
  1. Parses primary expressions (ident, `∅`, or parenthesized sub-expressions),
  2. Applies zero or more `'` (complement),
  3. Applies binary operator in correct order

The result is an AST with nodes (draft, may change later):

- `Ident(String)`, `Empty`
- `Unary::Complement(expr)`
- `Binary::Union/Intersect/Diff/SymDiff(lhs, rhs)`

## How Results Are Used

An interpreter that can evaluate the AST using an environment mapping identifiers to concrete sets (for example, `HashSet<i32>`) will be included. This will help to:

- define and store sets (`let A = {1,2,3};`)
- compute results of set expressions,
- print results (`print A ∪ B;`)

## Quick usage

From the root:

```sh
cargo run -- parse <path-to-file>
```
