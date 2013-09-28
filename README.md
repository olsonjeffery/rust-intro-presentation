# The Rust Programming Language

### (an introduction)

This is the notes README for an introductory presentation, on the topic of the [Rust Programming Language](http://rust-lang.org).

All code for this presentation is in the [`/src` directory](http://github.com/olsonjeffery/rust-intro-presentation/tree/master/src) of this repo.

    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    RRRRR    UU   UU   SSSSS   TTTTTT
    RR   RR  UU   UU  SS   SS    TT
    RR   RR  UU   UU   SSS       TT
    RRRRR    UU   UU     SSS     TT
    RR  RR   UU   UU  SS   SS    TT
    RR   RR   UUUUU    SSSSS     TT
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

# Why Rust?

### Safe

  - The compiler has rules and heuristics to ensure that all references to values are valid. No wild or unitialized pointers. No use-after-free
  - Rust has an unsafe subset of the language to
  handle FFI and other situtions where audit risk should be highlighted

### Concurrent

  - Shared-nothing concurrency
  - Race-free shared memory if needed
  - Rust code, by default, runs in Tasks atop a Scheduler, which owns an OS thread. All tasks within a single Scheduler are cooperative.
  - Tasks yield when entering IO or doing messaging. An explicit yield feature is imminent
  - Async/blocking IO is supported, out-of-the-box, atop libuv (the underlying IO layer for node.js)

### Practical

  - A rich typesystem
  - Rust has C-compatible struct layouts!
  - Rust features several categories of types more commonly found in functional languages
  - algebraic datatypes (enums or tagged unions)
  - tuples
  - traits (typeclasses in haskell). Rust has generics!
  - Nothing particularly innovative about the language. Most features follow existing academic work/features.
  - Like C/C++, can reason about semantics of memory layout in a reasonably transparent fashion. He who controls memory layout controls the universe.

# Random Trivia

- 0.8 was just released on thursday
- Rust has 220+ entries in its AUTHORS.txt file
- 60-70 PRs merged a week
- compiled, statically typed
- Language is block-based & curly-braced, (pretty much) no sig. whitespace
- Production-quality LLVM backend to emit machine code
  - LLVM is an open source project, with major backing by Apple, Google and others.. it's the backing infrastrufture for clang (the compiler for iOS)

# Modelling data

- enums, at first glance, might look their C counterparts, but they can be much more
- a single instance could be holding any of its
variants, but that isn't known until you try to
access the data
- enums can only be access by "destructuring" them
in a match

# structs: C-compatible memory layout

- show `uvll.rs`
- very important for FFI scenarios
  
# Traits: static/dynamic dispatch scenarios

### OLD


# Safe, concurrent, practical
- `/src/practical/hello.rs`

## Safe

* Won't slash your tires
* Won't kick your door in
* (Hopefully) No segfaults, wild pointers, data races, etc

## Pratical

* *Good FFI*
  - to include C-compatible struct layouts
  - uvll.rs
* Reasoning about memory layout
  - Stack, heap, dynamic memory pools, etc

`/code/src/dip/two.rs`

## Concurrent

* Easy to reason about memory and concurrency
