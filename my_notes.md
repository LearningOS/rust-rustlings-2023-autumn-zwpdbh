# Cheatsheet

## String

- Append some string
  - Keep original string, use `format`

# Problems to review

## How to handle errors

- errors5.rs
- errors6.rs

## How to use generic

- generics2.rs
  - How to define a generic struct
  - How to implement generic method for it.

## How to use trait

- trait2.rs

  - Notice the signature differences for "mut".

- trait4.rs

  - use trait as

- trait5.rs
  - Specifying Multiple Trait Bounds with the + Syntax

## How to specify lifetime

- lifetimes1.rs
- lifetimes2.rs

# How to handle panic

- About `panic::catch_unwind`
  - `panic::catch_unwind` typically returns a Result type, where Ok contains the result of the closure if it didn't panic, and Err contains the panic payload (an std::boxed::Box<dyn Any + Send>).
  - By applying .ok() to the result, you convert it into an Option:
    - If the result is an Ok, you get Some(result) with the result of the closure.
    - If the result is an Err (i.e., a panic occurred), you get None.
