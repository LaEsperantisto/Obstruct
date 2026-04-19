# Obstruct

A terse, statically-typed programming language written in Rust that transpiles to C (then compiled with `gcc`).

---

## Variable Declarations

- Immutable variables:

```Obstruct
#my_var = 0; // equivalent to Rust: let my_var = 0;
```

- Mutable variables:

```Obstruct
#@my_var = 1.1; // equivalent to Rust: let mut my_var = 1.1;
```

- Declaring without assigning (requires a type):

```Obstruct
#my_var: vec; // vec defaults to empty
#my_var2: i32; // defaults to 0
```

_**You must give the type, a value, or both!**_

---

## Control Flow

### Conditional Statements

- `if` statement:

```Obstruct
? condition {
    // do something
}
```

- `else if` statement:

```Obstruct
~? other_condition {
    // do something else
}
```

- `else` statement:

```Obstruct
~ {
    // fallback action
}
```

### Loops

- `while` loop (using `£`):

```Obstruct
£ condition {
    // loop body
}
```

- `for` loop:

```Obstruct
for i : expr {
    // loop body
}
```

---

## Print

Printing is a statement, not an expression.

- `$` prints without newline (like `print(end="")` in Python):

```Obstruct
$"Hello, world!";
```

- `$$` prints with a newline:

```Obstruct
$$1;
$2;
```

---

## Functions

- Defining a function:

```Obstruct
fn my_func(arg1: type, @arg2: type) return_type {
    // function body
}
```

If there is no return type, it can be omitted:

```Obstruct
fn print_num(n: i32) {
    $n;
}
```

- Returning from a function uses `ret`:

```Obstruct
fn add(a: i32, b: i32) i32 {
    ret a + b;
}
```

- Calling a function:

```Obstruct
my_func(arg1);
```

- Lambdas use `lam`:

```obstruct
#main = lam {
  $"Hello from a lambda!";
};
```

Lambdas are called like functions: `main()`.

---

## Generics

Generics use double arrows (`<<` and `>>`):

```obstruct
fn <<T>> push(v: vec<<T>>, item: T) vec<<T>> {
    // function body
}
```

Call generic functions with `<<T>>` syntax: `push<<i32>>(v, 5)`. The generic type can often be inferred.

---

## Data Types

- `i32` — 32-bit integer (default integer type)
- `f64` — 64-bit float
- `bool` — boolean (` `t` / `f`)
- `char` — single character (UTF-8)
- `strlit` — string literal (C `char*`)
- `vec<T>` — resizable vector
- `arr` — nil / empty array (`[]` type)
- `[]` — syntactic sugar for `arr`

### Literals

- Numbers: `42` (int), `3.14` (float)
- Strings: `"Hello"` (with `\\n`, `\\t`, `\\r`, `\\\\`, `\\"` escapes)
- Characters: `'c'` (with `\\n`, `\\t`, `\\r`, `\\\\`, `'\\''` escapes)
- Booleans: `` `t `` (true), `` `f `` (false)
- Empty string: `` `s ``
- Vectors: `\{1, 2, 3}`
- Arrays: `[1, 2, 3]`

---

## Data Structures

- `cls` — declare a class/struct using `cls`.

```obstruct
struct Point {
    x: f64,
    y: f64,
}

cls Circle {
    center: Point,
    radius: f64,
}
```

---

## Module System

Import other `.obs` files:

```Obstruct
use "path/to/file.obs";
use std "math.obs"; // from the std/ directory
```

---

## Built-in Functions

The following functions are registered at compile time. All operators are implemented via internal typed functions (
e.g., `_add`, `_sub`, `_less`) that are generated per type combination.

### Input

- `intput() -> i32` — reads an integer from stdin (note: named `intput`)
- `fput() -> f64` — reads a float from stdin
- `strput() -> strlit` — reads a string from stdin

### Print (internal, invoked automatically via `$` / `$$`)

`_print` is overloaded for: `i32`, `f64`, `bool`, `strlit`

### Comparison

Comparison operators are overloaded for `i32`, `f64`, and `strlit` (for `==`/`!=`):

- `==` (equal)
- `!=` (not equal)
- `<` (less)
- `>` (greater)
- `<=` (less or equal)
- `>=` (greater or equal)

### Arithmetic

- `+` — addition (`i32`, `f64`)
- `-` — subtraction (`i32`, `f64`)
- `*` — multiplication (`f64` only)
- `/` — division (`f64` only)
- `**` — exponentiation (`f64` only, uses C `pow()`)

Note: `%` (modulo) is parsed but not yet transpiled. `&&` / `||` (and / or) are parsed but not yet transpiled.

---

## Reference Syntax

The `&` and `*` operators are recognized in expressions:

- `&name` — resolves to a call to `ref::new(name)` (not currently a registered builtin)
- `*expr` — resolves to a call to `ref::deref(expr)` (not currently a registered builtin)

---

## Delete

`del` removes a variable from the compile-time environment:

```obstruct
#foo = 2;
del foo;
```

---

## Main

Every Obstruct program needs a `main` function as the entry point. The transpiler generates `main()` with no arguments
in C.

---

## Example Program

```Obstruct
fn main() {
    #x = 10;
    #@y = 5.0;

    ? x > 5 {
        y = y + 1.0;
    } ~? x == 5 {
        y = y - 1.0;
    } ~ {
        y = 0.0;
    };

    for i : x {
        // loop body
    };

    #result = add(x, y);
    $$result;

    ret;
};

fn add(a: i32, b: i32) i32 {
    ret a + b;
};
```

---

## How It Works

Obstruct compiles through these stages:

1. **Scanner** — tokenizes source into tokens (including Obstruct-specific symbols)
2. **Parser** — recursive descent parser builds an AST
3. **Pre-transpile** — registers functions, resolves `use` imports, generates declarations
4. **Transpile** — converts AST to C code (operators become typed function calls)
5. **C output** — combines includes, typedefs, operator implementations, and function bodies
6. **gcc** — compiles the C code with `-lm`

Binary operators are lowered to typed function calls (`_add`, `_sub`, `_less`, etc.) at transpilation time, allowing
type-specific C implementations to be generated for each type combination.
