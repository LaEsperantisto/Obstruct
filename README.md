# Obstruct

This is a simple, expressive programming language designed for rapid development with
minimal syntax. It combines elements of Rust-like structure with custom shorthand for
variable declarations, control flow, and functions.

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

- Declaring without assigning:

```Obstruct
#my_var: Vec; // equivalent to Rust: let my_var = Vec::new();
#my_var2: i32; // equivalent to Rust: let my_var = 0;
```

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

- `while` loop:

```Obstruct
£ condition {
    // loop body
}
```

- `for` loop (just like Rust):

```Obstruct
for i in 0..5 {
    // loop body
}
```

---

## Print

In Obstruct, printing text onto the screen / terminal is not a function, but rather a
statement. The equivalent of `print()` in many languages (or `print(end="")` python) is
`$` in Obstruct:

```Obstruct
$"Hello, world!";
```

However, the equivalent of `println()` (or `print()` in python) is instead `$$`:

```Obstruct
$$1;
$2;
```

As shown above, the `$` sign is succeeded by an expression, which will be printed onto
the screen / terminal. Even though this is a statement, not an expression, it does
however return the value of the expression.

---

## Functions

- Defining a function:

```Obstruct
fn my_func(arg1: type, @arg2: type) return_type {
    // function body
}
```

Note that when there is no return type, the return_type can be removed, e.g.

```Obstruct
fn print_num(n: i32) {
    $n;
}
```

- Returning from a function:

The keyword `ret` is used to return from a function

- Calling a function:

```Obstruct
my_func(arg1);
```

- Notes:
    - @ before an argument marks it as mutable.

    - Functions can have typed parameters and a return type.

---

## IMPORTANT!

### Block statements

Block statements, or scopes, work just like in Rust: you can have as many statements as
you want, each one ending with a semicolon, but only the _very last_ statement in a block
statement (as long as it is also an expression) may not end with a semicolon. This value
is the value that is returned by the block statement, making them expressions, not
statements.


---

## Data Structures

- `Vec`:

A Vec is a resizable list, similar to Rust's Vec. When a variable is declared as a Vec,
but no value is assigned, it defaults to an empty Vec.

- `arr`:

An `arr`, aka `Array` in some languages, is declared like this: `[val1, val2]` and is not resizable.
The size also needs to be known at compile type. The type of an Array is "declared" similar to Rust,
but with brackets instead of parentheses, e.g. `[i32, i32]` for the position of a 2D object. Similar
to Rust, The `nothing` type is `[]`. However, instead of using `[i32,f64]`, you could use `arr<i32,f64>`.
Again, this could be instead said as `arr` or `arr<>`.

- `str`:

A `str`, aka `String` in some languages, is declared with double quotes, like this: `"Hello, world!"` and is
resizable. When a variable is declared as a str, but no value is assigned, it defaults to
an empty str.

- `i32`:

Integers in Obstruct are not said `int`, but rather, like Rust, as `i32` or `i64`. `i32`
Is the default `int` in Obstruct, and the most used, however, you can also use any power
of 2 from 8 to 64, like in Rust.

- `char`:

Single character are `char`s, which are UTF-8 encoded. Basic escape sequences supported,
like `\n`, `\t`, `\\`, and `\'`. These are also valid for `str` literals, but the only
difference is that `\'` is invalid, only `\"` is supported in `str` literals.

---

## Main

Every Obstruct program needs to have a `main` function, which is the entry point of the
program. The `main` function takes _one_ argument, which is the arguments the program
was called with, in the form of a `Vec<str>`. The main function can return either nothing,
which means that if there is no error, the exit code will be `0`, but it can return an i32,
which will be the exit code. If at any point there is an error, the exit code will be `1`.

---

## Builtin Functions

- `quit()`:

This function is a manual exit, and should only be used for emergency crashes. This bypasses
all destructors.

---

## Summary

This language aims to:

- Reduce boilerplate for common programming patterns.

- Make variable declaration and mutability clear with simple syntax.

- Provide familiar Rust-style loops and functions for easy adoption.

- Offer concise conditional and branching statements.

## Example program

```Obstruct
fn main(args: Vec<str>) {
    #x = 10;
    #@y = 5.0;
    
    ? x > 5 {
        y = y + 1.0;
    } ~? x == 5 {
        y = y - 1.0;
    } ~ {
        y = 0.0;
    };
    
    #temp: Vec<i32>;
    for i in 0..x {
      temp.push(i);
    };
    
    #result = add(x, y);
    $result;
    
    ret;
};

fn add(a: i32, @b: i32) i32 {
    b = a + b
    b
};
```