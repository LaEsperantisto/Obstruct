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

_**You must give the type, a value or both!**_

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

- Lambda:

The keyword `lam` is used for declaring lambdas (I checked the plural), like this:

```obstruct
#main = lam {
  $"Hello from a lambda!";
};

#push = lam <<T>> (v: vec<<T>>, item: T) vec<<T>> {
    vec::push(v, item)
};
```

Lambdas are called just like functions: `main()`.

- Notes:
    - @ before the function name marks it as mutable.

    - Functions and lambdas can have typed parameters and a return type.

---

## IMPORTANT!

### Block statements

Block statements, or scopes, work just like in Rust: you can have as many statements as
you want, each one ending with a semicolon, but only the _very last_ statement in a block
statement (as long as it is also an expression) might not end with a semicolon. This value
is the value that is returned by the block statement, making them expressions, not
statements.

---

## Generics

Generics in Obstruct work similar to Rust and Java. You can declare generic functions and
types just like in Rust:

```obstruct
fn <<T>> push (v: vec<<T>>, item: T) vec<<T>> {
    vec::push(v, item)
};
```

*Note that generics use **2** arrows (`<<` and `>>`) not just one.*

To call generic functions, you call them just like normal functions, but with the double
arrows: `push<<i32>>(v, 5)`. The generic is not always necessary.

---

## Data Structures

- `Vec`:

A `Vec` is a resizable list, similar to Rust's Vec. When a variable is declared as a Vec,
but no value is assigned, it defaults to an empty Vec.

- `arr`:

An `arr`, aka `Array` in some languages, is declared like this: `[val1, val2]` and is not resizable.
The size also needs to be known at compile type. The type of an Array is "declared" similar to Rust,
but with brackets instead of parentheses, e.g. `[i32, i32]` for the position of a 2D object. Similar
to Rust, The `nothing` type is `[]`. However, instead of using `[i32,f64]`, you could use `arr<i32,f64>`.
Again, this could be instead said as `arr` or `arr<>`.

- `ptr`:

A `ptr` is a `pointer` to memory. It can currently only point to the heap. To dereference a pointer, call
the `deref` function.

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

## Delete

You can `del` variables, similar to how rust can `drop` variables, like this:

```obstruct
#foo = 2;
$foo;       // works, foo exists
del foo;
$foo;       // error, foo doesn't exist anymore
```

However, to delete raw pointers, call the `ptr::free` function (see below).

---

## Pointers

To allocate memory, call the `ptr::new` function (see in Builtin Functions section below). To access
a pointer's value, call the `ptr::deref` function, and to free a value, call the `ptr::free` function.

---

## Builtin Functions

- `fn quit`:

This function is a manual exit, and should only be used for emergency crashes. This bypasses
all destructors.

- `fn in str`:

This is the standard input, that returns a `str` from the input.

- `fn <T> vec::push(v: vec<T>, item: T) vec<T>`:

Adds the item to the back of the `vec` by returning the new `vec`.

- `fn <T> len(i: T) i32`:

This function returns the length of `i`, as an `i32`.

- `type`:

This is a function that returns the type of the argument (requires _one_ argument). I didn't
include the `fn`, etc. since this is functionally impossible without it being native.

- `fn init_window`:

This is a function that initializes a window, using the standard game engine, `cobject`.

- `fn draw_window`:

This function updates the window, and draws all the objects.

- `fn is_window_open bool`:

This returns true if the window is open, and false if it is closed.

- `fn <T> ptr::deref(pointer: ptr<T>) T`:

This function dereferences pointers pointing to something on the heap.

- `fn <T> ptr::new(value: T) ptr<T>`:

This function allocates memory on the heap, and returns a pointer with the return address.

- `fn <T> ptr::free(pointer: ptr<T>)`:

This function frees allocated memory from the heap. The pointer is then no longer usable.

- Constructors:

All the default types have constructors (`type::new`).

- Operators:

All the default types overload the operators, like `i32` + `i32`, but not `str` * `str`.

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
    
    #temp: vec<i32>;
    for i in 0..x {
      temp.push(i);
    };
    
    #result = add(x, y);
    $result;
    
    ret;
};

fn add(a: i32, b: i32) i32 {
    a + b
};
```