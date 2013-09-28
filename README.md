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
  - Reasoning about memory layout
    - Stack, heap, dynamic memory pools, etc
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

- explain code example
- uses of `input` in `good_times` are statically dispatched
- show example of dynamic dispatch

# Memory

- pretty self-explanatory, in terms of allocation
- mention variable shadowing
- Unique boxes must satisfy the `Send` trait:
  - No references
  - No managed boxes
  - They have a "linear" lifetime that is known to the compiler
- Managed boxes are task-local. The box can be cheaply copied.
- Current scheme of reference-counting w/ cycle-collection.. worst case: memory is freed at the end of the tasks
- Real GC is coming
- Managed pointers will probably become a library type

# Memory (arrays of data)

- This is a vector, the rust-idiomatic means of representing contiguous, homogeneous data
- Different allocation scenarios dictate its layout
- "growable vecs" double in size as they grow

# Quick aside on mutability

- Worth noting is that rust makes mutability an explicit choice on the part of the programmer. all values are immutable by default. Mutability is specified at time of storage and is inherited.
- If you return a value and absolutely want to make some part of its data immutable, use private fields

# Ownership

- What is ownership?
- The right to free memory
- note that parens are option in if, et al statements
- brackets are never optional
- In this example, ownership of `bar` has been "moved" into `foo()`

# Ownership, again

- Here, foo gets a reference to `bar` and doesn't "own" it

# Lifetimes

- Lifetimes are a very complex topic, worthy of their own, standalone presentation
- Here is a visualization of lifetimes, as viewed by
the compiler
- They allow getting references to some value in a data structure and returning it to a caller, *provided* the compiler can verify that the "borrowed-reference" doesn't outlive the lifetime of the value being borrowed-from.

# Tasks

- Tasks are the rust unit of execution
- The scheduler does not pre-empt running tasks
- Tasks yield in certain scenarios, like entering IO or messaging

# Schedulers

- Schedulers map 1:1 with an OS Thread
- N tasks are multiplexed onto the Scheduler
- Schedulers can steal work from other Schedulers if they have nothing to do
- IO is an exception to this rule (at least under libuv)

# IO

- The current scheduler and IO is implemented atop libuv, the backing IO layer for node.js
- The user-facing IO API is presented as being blocking
- But it is async under the hood

# Programming In The Large

- Software programming, at it's core, is about solving discrete problems using the tools at hand.
- Some tools are better suited to tasks at different scale.
- Think about the differences between shell-script and a ruby-on-rails project
- Programming-in-the-large speaks to this. It's about working on projects of a larger scale, usually consisting of tens, hundreds or thousands of individual components and tasks, with possibly several teams working on a shared codebase independant of each other.
- Rust endevours to provide tooling to support this

# Test framework

- rust provides tools to embed tests alongside your code or in other modules
- code compiled with the "test" flag will always output an executable, which when ran will execute available tests

# Modules

- Rust has a modern, sophisticated module system
- Here we have an example of several modules within a single project
- Based on the structure, you can that modules can be nested within folders or placed alongside each other
- "use" statements make code in other modules available within the current module
- "mod" statements provide links to modules
- "extern mod" refers to an externally linked module
