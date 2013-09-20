# The Rust Programming Language

Blah, blah

# Agenda

- Front Matter
  * Some quick background on Rust
- The Rust Lexicon By Example
  * A crash course in the language of the language
- Crucial Semantics
  - Type System
  - Memory
  - Runtime/Scheduler/IO
- Programming-in-the-large
  * How Rust can help you think big
  
## My Goal

If one person who hears this talk comes away intrigued enough about
Rust to earnestly attempt to learn more and become a contributor, I'll be happy.

There's too much to cover here. My greatest apprehension is providing too
shallow of a treatment of the many subtle, sophisticated topics that make
up the whole of Rust.

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

## structs

Look a lot like C-structs. They have a matching memory layout, too. You can map
a C struct in rust code, pass a reference to that struct (maybe you keep it on the
stack, maybe in a heap, etc) into a C function and have it work.

TODO: ex-structs.rs

## enums

## tuples

Immutable collections of arbitrary values. Can be destructured. Single return?
Who cares.

TODO: ex-tuples.rs

## mutability

TODO: ex-mutability.rs

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

# Programming-in-the-large

Small, isolated tasks are great and all. But at the end of the day, we're here to solve real world problems. And often, those problems get complicated by the baggage of scaling out a solution on any given technology platform.

## A module system

Beating up on C & C++ header files is too easy.

[ex-modules]
