# The Rust Programming Language

### (an introductory presentation)

This is the notes README for a reveal.js-based presentation, on the topic of the [Rust Programming Language](http://rust-lang.org) contained in this repo. The presentation can be viewed online [here](http://olsonjeffery.github.io/rust-intro-presentation)

All code for this presentation is in the [`/code` directory](http://github.com/olsonjeffery/rust-intro-presentation/tree/master/code) of this repo.

The original reveal.js `README.md` has been removed/replaced with this document. Otherwise this repo is mostly like the reveal.js repo. It's original license information is available in the provided `LICENSE.txt`.

# Safe, concurrent, practical
- `/code/src/dip/zero.rs`

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
