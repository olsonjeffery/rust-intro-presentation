// Copyright 2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.

/*! Who is Rust For?

* We have our sights set on C++, perf-wise

*/

/// With padding by default (`more`...)
///
/// ```rust
/// struct Bar { 
///     name: String
/// }
///
/// struct Foo {
///     baz: int,
///     is_froboz: bool,
///     children: Vec<Bar>
/// }
/// ```
/// 
/// Memory layout is static, like C & C++
///
/// ```ignore
/// struct Foo {
///    bar: Foo // BZZZT! Wrong.. cyclic dependency
/// }
/// ```
pub fn c_compatible_memory_layout() {
}

/// Comparable in sophistication/robustness to C/C++ (more...)
///
/// ```rust
/// let a_word = "bar";                           // stack allocated, const-sized str value 
/// let some_number = 42u;
/// let some_numbers: [u8, ..3] = [1, 2, 3];      // stack-allocated, fixed-size
///                                               // array value
/// let some_numbers: Vec<u8> = vec!(1, 2, 3, 4); // heap-allocated
/// ```
pub fn fine_grained_memory_control() {
}

/// Can run without a runtime (free-standing)
///
/// "Placement new" is forthcoming, as well.
///
/// ```
/// use std::rc::Rc;
/// let foo = 1u;            // stack allocated
/// let bar = box 1u;        // type Box<uint>
/// let bar = Rc::new(1u);   // type Rc<uint>
/// let weak_bar = bar.downgrade();
pub fn pay_for_what_you_use() {
}
