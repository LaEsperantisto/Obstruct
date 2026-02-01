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

## Functions

- Define a function:

```Obstruct
fn return_type my_func[arg1: type, @arg2: type] {
    // function body
}
```

Note that when there is no return type, the return_type can be removed, e.g.

```Obstruct
fn print_num[n: i32] {
    $n;
}
```

- Returning from a function:

The keyword `ret` is used to return from a function

- Call a function:

```Obstruct
my_func(arg1);
```

- Notes:
    - @ before an argument marks it as mutable.

    - Functions can have typed parameters and a return type.

---

## Data Structures

- Vec:

A Vec is a resizable list, similar to Rust's Vec. When a variable is declared as a Vec,
but no value is assigned, it defaults to an empty Vec.

- Array:

An Array is declared like this: `[val1, val2]` and is not resizable. The size also needs
to be known at compile type.

- str:

A str, aka `String`, is declared with double quotes, like this: `"Hello, world!"` and is
resizable. When a variable is declared as a str, but no value is assigned, it defaults to
an empty str.

---

## Main

Every Obstruct program needs to have a `main` function, which is the entry point of the
program. The `main` function take in _one_ argument, which is the arguments the program
was called with, in the form of a `Vec<str>`. The main function can return either nothing,
which means that if there is no error, the exit code will be `0`, but it can return an i32,
which will be the exit code.

---

## Summary

This language aims to:

- Reduce boilerplate for common programming patterns.

- Make variable declaration and mutability clear with simple syntax.

- Provide familiar Rust-style loops and functions for easy adoption.

- Offer concise conditional and branching statements.

## Example program

```Obstruct
fn main[args: Vec<str>] {
    #x = 10;
    #@y = 5.0;
    
    ? x > 5 {
        y = y + 1.0;
    } ~? x == 5 {
        y = y - 1.0;
    } ~ {
        y = 0.0;
    }
    
    for i in 0..x {
        #temp: Vec;
    }
    
    let result = add(x, y);
    $result;
}

fn i32 add[a: i32, @b: i32] {
    return a + b;
}
```