//@ aux-build: issue-95236.rs
//@ ignore-cross-compile

extern crate issue_95236;

pub use issue_95236::ObligationCause;

// @has issue_95236/struct.ObligationCause.html '//*[@id="method.raise"]' 'fn raise'
