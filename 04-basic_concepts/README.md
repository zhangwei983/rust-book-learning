This sample shows some common programming concepts of Rust.

## Shadowing vs mut

Shadowing is different from marking a variable as `mut`. Shadowing allow changing the type of the value.

## Scalar Types

### Integer

| Length    |  Signed   | Unsigned   |
| --------- | --------- | ---------- |
| 8-bit     | i8        | u8         |
| 16-bit    | i16       | u16        |
| 32-bit    | i32       | u32        |
| 64-bit    | i64       | u64        |
| 128-bit   | i128      | u128       |
| arch      | isize     | usize      |

The isize and usize types depend on the architecture of the computer your program is running on, which is denoted in the table as “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

### Float  

Rust’s floating-point types are:
- f32  
- f64  

The default type is `f64` because on modern CPUs, it’s roughly the same speed as f32 but is capable of more precision. 

All floating-point types are `signed`.

### Boolean

Specified using `bool`.
- true
- false

Booleans are one byte in size.

### Character

Specified using `char`.

Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII.

https://doc.rust-lang.org/stable/book/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings

## Compound Types

Rust has two primitive compound types: `tuples` and `arrays`.

### The Tuple Type

- A tuple is a general way of grouping together a number of values with a `variety of types` into one compound type. 
- Tuples have a fixed length.
- The tuple without any values has a special name, `unit`. 


### The Array Type

- Every element of an array must have the same type.
- Arrays in Rust have a `fixed` length.
- Arrays are useful when you want your data allocated on the `stack` rather than the heap.
- If you want to change the length, use [vector](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html) instead.
- {:?} placeholder can be used to print a whole array.

## Function

Rust code uses `snake case` as the conventional style for function and variable names, in which all letters are `lowercase` and `underscores` separate words.

## Statements and Expressions

- `Statements` are instructions that perform some action and do not return a value.
- `Expressions` evaluate to a resultant value.

## Control Flow

if else if else

loop while for

For `loop`, you can set a `label` and use it along with break and continue, a label starts with `'`.

range loop