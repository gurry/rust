//@ only-windows
// Verify debug_annotation behavior: happy paths and error cases.

#![feature(debug_annotation)]
#![feature(core_intrinsics)]

use std::intrinsics::debug_annotation;

// Single annotation via intrinsic
fn intrinsic_single() {
    debug_annotation(&["test_annotation"]);
}

// Multiple annotations via intrinsic
fn intrinsic_multiple() {
    debug_annotation(&["category", "subcategory", "details"]);
}

// Single annotation via macro
fn macro_single() {
    std::hint::debug_annotation!("macro_test");
}

// Multiple annotations via macro
fn macro_multiple() {
    std::hint::debug_annotation!("Performance", "HotPath", "Critical");
}

// Error case: non-constant argument
fn non_const_arg() {
    let s = "hello";
    debug_annotation(&[s]); //~ ERROR debug_annotation requires constant string literal arguments
}

// Error case: function parameter
fn fn_param_arg(strs: &[&str]) {
    debug_annotation(strs); //~ ERROR debug_annotation requires constant string literal arguments
}

// Error case: named const
const MY_CONST: &str = "hello";
fn named_const_arg() {
    debug_annotation(&[MY_CONST]); //~ ERROR debug_annotation requires constant string literal arguments
}

// Error case: empty array
fn empty_array() {
    debug_annotation(&[]); //~ ERROR debug_annotation requires at least one string literal argument
}

fn main() {
    intrinsic_single();
    intrinsic_multiple();
    macro_single();
    macro_multiple();
    non_const_arg();
    fn_param_arg(&["a"]);
    named_const_arg();
    empty_array();
}
