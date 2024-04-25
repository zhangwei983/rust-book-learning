An example shows how to use lifetimes annotation in Rust.

The main aim of lifetimes is to prevent `dangling references`. Annotating lifetimes is not a concept most other programming languages have, so this is going to feel unfamiliar. 

## Preventing Dangling References

```rust
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
```

The above code won't compile by using a `borrow checker`.

## Borrow Checker

Borrow checker compares scopes to determine whether all borrows are valid. 

```rust
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

- The inner `'b` lifetime block is much smaller than the outer `'a` lifetime block.
- Rust compares the size of the two lifetimes and sees that `r` has a lifetime of `'a` but that it refers to memory with a `shorter` lifetime of `'b`. 
- The program is rejected.

## Generic Lifetimes in Functions

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

The above code can't compile as Rust can’t tell whether the reference being returned refers to x or y.

We also don’t know the concrete lifetimes of the references that will be passed in, so we can’t look at the scopes to decide the returned reference is valid or not.

To fix this error, we’ll add generic lifetime parameters that define the relationship between the references so the borrow checker can perform its analysis.

## Lifetime Annotation Syntax

Lifetime annotations 
- don’t change how long any of the references live.
- describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.

Functions can accept references with any lifetime by specifying a generic lifetime parameter.

Lifetime annotations have a slightly unusual syntax: 
- the names of lifetime parameters must start with an `apostrophe (')`.
- are usually all `lowercase` and very `short`, like generic types.
- is placed `after` the `&` of a reference, using a `space` to separate the annotation from the reference’s type.

Examples:
```
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

## Lifetime Annotations in Function Signatures

Below is an example of lifetime annotations in function signature.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

When annotating lifetimes in functions, the annotations go in the `function signature`, not in the function body. 

The above function signature tells Rust that: 
- the two parameters of `longest` function , both of which are string slices that live at least as long as `lifetime 'a`. 
- the string slice `returned` from the function will live at least as long as `lifetime 'a`.

This means that the lifetime of the reference `returned` by the `longest` function is the same as the smaller of the lifetimes of the values referred to by the function arguments. 

When we specify the lifetime parameters in this function signature, we’re:
- `not` changing the lifetimes of any values passed in or returned.
- `specifying` that the borrow checker should reject any values that don’t adhere to these constraints.

In the above example, the `longest` function doesn’t need to know exactly how long `x` and `y` will live, only that some scope can be substituted for 'a that will satisfy this signature.

```rust
let string1 = String::from("abcd");

    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    println!("The longest string is {}", result); // This won't compile.
```

The above code won't compile. 
- The error shows that for `result` to be valid for the `println!` statement, `string2` would need to be `valid` until the end of the outer scope. 
- Rust knows this because we `annotated` the lifetimes of the function parameters and return values using the `same` lifetime parameter `'a`.

## Lifetime Annotations in Struct Definitions

Structs can be defined to hold `references`, but in that case we would need to add a `lifetime annotation` on `every reference` in the struct’s definition.

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

The name of the generic lifetime parameter is declared inside angle brackets after the name of the struct so we can use the lifetime parameter in the body of the struct definition.
