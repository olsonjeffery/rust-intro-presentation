// Copyright 2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.

/*! Quick overview of the language/community
*/

/// C-like at first glance (more...)
///
/// But quite different in many ways!
///
/// ```rust
/// fn foo(bar: uint) -> bool {
///     true
/// }
/// 
/// let a_value = 42u;
/// let explicitly_typed: Option<bool> = Some(true);
/// let result = match explicitly_typed { // exhaustive match
///     Some(v) => v,
///     None => fail!("triggers unwinding")
/// };
/// ```
pub fn curly_braced_and_expression_based() {
    fn foo(bar: uint) -> bool {
        true
    }
    
    let a_value = 42u;
    let explicitly_typed: Option<bool> = Some(true);
    let result = match explicitly_typed { // exhaustive match
        Some(v) => v,
        None => fail!("triggers unwinding")
    };
}

/// TL;DR -- copyright assignment
pub fn released_under_the_apache_license() {
}

/// 0.11 was recently released, 0.12 in 3 months.. 1.0 by year's end
pub fn still_alpha_quality_software() {
}
