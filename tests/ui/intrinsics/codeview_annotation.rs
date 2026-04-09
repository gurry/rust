//@ only-windows
// Verify codeview_annotation behavior: happy paths and error cases.

#![feature(codeview_annotation)]
#![feature(core_intrinsics)]

use std::intrinsics::codeview_annotation;

// Single annotation via intrinsic
fn intrinsic_single() {
    codeview_annotation(&["test_annotation"]);
}

// Multiple annotations via intrinsic
fn intrinsic_multiple() {
    codeview_annotation(&["category", "subcategory", "details"]);
}

// Single annotation via macro
fn macro_single() {
    std::hint::codeview_annotation!("macro_test");
}

// Multiple annotations via macro
fn macro_multiple() {
    std::hint::codeview_annotation!("Performance", "HotPath", "Critical");
}

// Error case: non-constant argument
fn non_const_arg() {
    let s = "hello";
    codeview_annotation(&[s]); //~ ERROR codeview_annotation requires constant string literal arguments
}

// Error case: function parameter
fn fn_param_arg(strs: &[&str]) {
    codeview_annotation(strs); //~ ERROR codeview_annotation requires constant string literal arguments
}

// Error case: named const
const MY_CONST: &str = "hello";
fn named_const_arg() {
    codeview_annotation(&[MY_CONST]); //~ ERROR codeview_annotation requires constant string literal arguments
}

// Error case: empty array
fn empty_array() {
    codeview_annotation(&[]); //~ ERROR codeview_annotation requires at least one string literal argument
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
