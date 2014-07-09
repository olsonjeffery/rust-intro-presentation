// Copyright 2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.

/*! Why is Rust fun to code in?
*/

/// ADT, Pattern Matching, etc...
///
/// WRONG
///
/// ```ignore
/// let foo = false;
/// foo = true;
/// ```
///
/// Better, thank you.
///
/// ```
/// let mut foo = false;
/// foo = true;
/// ```
///
/// Statically ensured exhaustive pattern matching
/// 
/// ```rust
/// enum ConnectionStatus {
///     Error(String),
///     Connected(uint),
///     Connecting
/// }
/// 
/// let conn_status = Error("connection timed out".to_string());
/// match conn_status {
/// }
/// ```
pub fn major_ML_influences() {
}

/// Makes TDD a breeze
///
/// ```rust
/// // this is only built into test-configured builds
/// #[cfg(test)]
/// fn some_util_function() -> bool {
///     true
/// }
///
/// // this is a test
/// #[test]
/// fn util_is_available() {
///     some_util_function();
/// }
/// ```
pub fn built_in_test_and_benching_harnesses() {
}

pub fn travis_ci_infrastructure() {
}
