# Obstruct

A terse, statically-typed programming language written in Rust that transpiles to C (then compiled with `gcc`).

---

## Variable Declarations

- Immutable variables:

```obstruct
#my_var = 0; // equivalent to Rust: let my_var = 0;
```

- Mutable variables:

```obstruct
#@my_var = 1.1; // equivalent to Rust: let mut my_var = 1.1;
```

- Declaring without assigning (requires a type):

```obstruct
#my_var: i32; // defaults to 0
```

_**You must give the type, a value, or both!**_

---

## Control Flow

### Conditional Statements

- `if` statement:

```obstruct
? condition {
    // do something
}
```

- `else if` statement:

```obstruct
~? other_condition {
    // do something else
}
```

- `else` statement:

```obstruct
~ {
    // fallback action
}
```

### Loops

- `while` loop (using `£`):

```obstruct
£ condition {
    // loop body
}
```

- `for` loop:

```obstruct
for i : expr {
    // loop body
}
```

---

## Print

Printing is a statement, not an expression.

- `$` prints without newline (like `print(end="")` in Python):

```obstruct
$"Hello, world!";
```

- `$$` prints with a newline:

```obstruct
$$1;
$2;
```

---

## Functions

- Defining a function:

```obstruct
fn my_func(arg1: i32, @arg2: i32) i32 {
    // function body
}
```

Parameters prefixed with `@` are mutable inside the function body.

If there is no return type, it can be omitted:

```obstruct
fn print_num(n: i32) {
    $n;
}
```

- Returning from a function uses `ret`:

```obstruct
fn add(a: i32, b: i32) i32 {
    ret a + b;
}
```

- Calling a function:

```obstruct
my_func(arg1);
```

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
- Strings: `"Hello"` (with `\n`, `\t`, `\r`, `\\`, `\"` escapes)
- Characters: `'c'` (with `\n`, `\t`, `\r`, `\\`, `\'` escapes)
- Booleans: `` `t `` (true), `` `f `` (false)
- Empty string: `` `s ``

---

## Data Structures

- `cls` — declare a class/struct using `cls`.

```obstruct
cls Circle {
    center_x: f64,
    center_y: f64,
    radius: f64,
}
```

---

## Module System

Import other `.obs` files:

```obstruct
use "path/to/file.obs";
use std "math.obs"; // from the std/ directory
```

---

## Built-in Functions

The following functions are registered at compile time. All operators are implemented via internal typed functions (e.g., `_add`, `_sub`, `_less`) that are generated per type combination.

### Input

- `intput() -> i32` — reads an integer from stdin (note: named `intput`)
- `fput() -> f64` — reads a float from stdin
- `strput() -> strlit` — reads a string from stdin

### Comparison

Comparison operators are overloaded for `i32` and `f64`:

- `==` (equal) — also works for `strlit` and `char`
- `!=` (not equal) — also works for `strlit` and `char`
- `<` (less)
- `>` (greater)
- `<=` (less or equal)
- `>=` (greater or equal) — also works for `char`

### Arithmetic

- `+` — addition (`i32`, `f64`, `strlit`; also `strlit` + `char`)
- `-` — subtraction (`i32`, `f64`)
- `*` — multiplication (`i32`, `f64`)
- `/` — division (`i32`, `f64`)
- `**` — exponentiation (`i32`, `f64`, uses C `pow()`)
- `&` — logical and (also `&&`)
- `|` — logical or (also `||`)
- `!` — logical not

### Reference Operators

The `&` and `*` operators produce C reference/dereference expressions:

- `&name` — generates C `&name` (reference to variable)
- `*expr` — generates C `(*expr)` (dereference expression)

---

## Delete

`del` removes a variable from the compile-time environment:

```obstruct
#foo = 2;
del foo;
```

---

## Main

Every Obstruct program needs a `main` function as the entry point. The transpiler generates `main()` with no arguments in C.

---

## Example Program

```obstruct
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

Binary operators are lowered to typed function calls (`_add`, `_sub`, `_less`, etc.) at transpilation time, allowing type-specific C implementations to be generated for each type combination.

---

## Known Issues

- `%` (modulo) — parsed but not yet transpiled (will panic during transpilation)
- `for` loop — parsed but not yet transpiled
- `lam` (lambdas) — parsed but not yet transpiled
- `\{1, 2, 3}` (vector literals) — parsed but not yet transpiled
- `[1, 2, 3]` (array literals) — parsed but not yet transpiled
- `vec<T>` (variable-length vectors) — type exists but not fully implemented
- `ref` types — `&`/`*` emit C reference expressions but cannot be used as first-class values (not yet registered as builtin functions)
- `cls` (classes with inheritance/overrides) — basic struct definition works; `stc` and `ovr` keywords recognized but not fully implemented
