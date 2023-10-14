# Cheatsheet

## String

- Append some string

  - Keep original string, use `format`

- From other parts
  - iterator2.rs

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

## How to handle panic

- About `panic::catch_unwind`
  - `panic::catch_unwind` typically returns a Result type, where Ok contains the result of the closure if it didn't panic, and Err contains the panic payload (an std::boxed::Box<dyn Any + Send>).
  - By applying .ok() to the result, you convert it into an Option:
    - If the result is an Ok, you get Some(result) with the result of the closure.
    - If the result is an Err (i.e., a panic occurred), you get None.

## Map and Reduce

- iterator2.rs

  - It also shows examples about how to capitalize first character of a string.

- iterator3.rs
  - `result_with_list` could be used to if you want to propagate the first encountered error and return a single result
  - `list_of_results` could be used to collecting all the results and preserving the individual errors.
  - **Understand** `collect`, how the type you declare for values affects how the collect method processes the iterator's output.
- iterator4.rs
- iterator5.rs

# Concurrency

## Mutex

## Arc

- arc1.rs

## Cow

- cow1.rs ???

## Thread

- threads1.rs
- threads2.rs (Mutex + Arc)
- threads3.rs
  - Run `rustlings hint threads3` use some examples to tell us a summary of owership.
  - multiple-producer, single-consumer
  - Solution:
    - clone sender to multiple senders when then use cloned senders in different threads.
