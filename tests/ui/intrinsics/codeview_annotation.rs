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

// Named const elements
const ANNOTATION_A: &str = "hello";
const ANNOTATION_B: &str = "there";

fn named_const_elements() {
    codeview_annotation(&[ANNOTATION_A, ANNOTATION_B]);
}

// Named const and literal
fn mixed_const_and_literal() {
    codeview_annotation(&[ANNOTATION_A, "world"]);
}

// Const slice
const STRS_SLICE: &[&str] = &["hello", "there"];

fn named_const_slice() {
    codeview_annotation(STRS_SLICE);
}

// Ref to named const array
const STRS_ARRAY: [&str; 2] = ["hello", "there"];

fn named_const_array_ref() {
    codeview_annotation(&STRS_ARRAY);
}

// Error case: local variable
fn non_const_arg() {
    let s = "hello";
    codeview_annotation(&[s]); //~ ERROR codeview_annotation requires constant string literal arguments
}

// Error case: function parameter
fn fn_param_arg(strs: &[&str]) {
    codeview_annotation(strs); //~ ERROR codeview_annotation requires constant string literal arguments
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
    named_const_elements();
    mixed_const_and_literal();
    named_const_slice();
    named_const_array_ref();
    non_const_arg();
    fn_param_arg(&["a"]);
    empty_array();
}
