# The Rust Programming Language

### (an introductory presentation)

This is the notes README for a reveal.js-based presentation, on the topic of the [Rust Programming Language](http://rust-lang.org) contained in this repo. The presentation can be viewed online [here](http://olsonjeffery.github.io/rust-intro-presentation)

The original reveal.js `README.md` has been removed/replaced with this document. Otherwise this repo is mostly like the reveal.js repo. It's original license information is available in the provided `LICENSE.txt`.

# Agenda

- Front Matter
  * Some quick background on Rust
- The Rust Lexicon By Example
  * A crash course in the language of the language
- Runtime/Scheduler/IO
  * A bit about the rust runtime and the scheduler
- Programming-in-the-large
  * How Rust can help you think big

## How active is Rust as a project?

- ~220 entries in AUTHORS.txt
- 60-70 PRs merged a week
- Issue tracker closing in on #10000
- 0.1 released in January, 2001
- 1.0 by the end of 2013 (perhaps)

non-code minutae

## Hello world!

alright, I fibbed. here's some code

# The Rust Lexicon By Example

give you some rust all up in yer face

## data structures

- structs
  * C-compatible memory layout
- enums
  * algebraic data types, familiar to ocaml, f#, haskell, etc hackers. tagged unions in C
- tuples
  * immutable collections of arbitrary values
  * inner values can only be accessed by a match statement or destructuring-let
- tuples structs
  * like tuples, except that they have a name
- single-field tuple structs, aka "newtypes"
  * useful for wrapper a given type, so that

TODO: 1-data-structures.rs

## by-val, by-ref & pointers

- values
- `*`
- `~`
- `@`
- `&`
- moves, consumes, implicit copies

- by-val parameters in function signatures *always* copy
- `*` is an unsafe pointer. These are just used for C interop. If you're using
them in normal code, it's because you can't quit pointer arithmetic or something
other pattern that you can't acheive in idiomatic and safe rust code. Go back to
C or C++.
- `~` is a unique pointer. They're sendable, non-copyable and their value is in
a shared heap known as the "exchange heap"
- `@` is a "managed reference", and they point into a task-local heap. They are
reference counted and cycle collected. Worst case scenario: They're free'd when
the task ends. Real GC is coming. The goal is to move @ out of the compiler and
into a library, probably w/ `@` remaining as sugar
- `&` is a reference, or "borrowed pointer" in the docs. But it's just a reference.
It's always going to point to some value stored somewhere else (be it on the heap
or the stack) and the compiler will ensure that it doesn't outlive the value
referenced.

# traits & impls

TODO: ex-traits.rs

## mutability

TODO: ex-mutability.rs

# Programming-in-the-large

Small, isolated tasks are great and all. But at the end of the day, we're here to solve real world problems. And often, those problems get complicated by the baggage of scaling out a solution on any given technology platform.

## A module system

Beating up on C & C++ header files is too easy.

[ex-modules]
