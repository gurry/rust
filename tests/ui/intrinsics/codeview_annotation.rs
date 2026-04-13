// Verifies codeview_annotation compile-time behavior:
// - Happy paths: inline literals, macro usage, named const elements,
//   mixed const/literal, const slices, const array refs, and const fn usage.
// - Error cases: non-const arguments, function parameters, empty arrays,
//   and wrong types.
#![feature(codeview_annotation)]
#![feature(core_intrinsics)]

use std::intrinsics::codeview_annotation;

fn intrinsic_single() {
    codeview_annotation(&["string1"]);
}

fn intrinsic_multiple() {
    codeview_annotation(&["string1", "string2", "string3"]);
}

fn macro_single() {
    std::hint::codeview_annotation!("string1");
}

fn macro_multiple() {
    std::hint::codeview_annotation!("string1", "string2", "string3");
}

const ANNOTATION_A: &str = "string1";
const ANNOTATION_B: &str = "string2";
const ANNOTATION_C: &str = "string3";
fn named_const_elements() {
    codeview_annotation(&[ANNOTATION_A, ANNOTATION_B, ANNOTATION_C]);
}

fn mixed_named_const_and_literal_elements() {
    codeview_annotation(&[ANNOTATION_A, "string2", "string3"]);
}

const STRS_SLICE: &[&str] = &["string1", "string2", "string3"];
fn named_const_slice() {
    codeview_annotation(STRS_SLICE);
}

const STRS_ARRAY: [&str; 3] = ["string1", "string2", "string3"];
fn named_const_array_ref() {
    codeview_annotation(&STRS_ARRAY);
}

// Use in const function
const fn annotated_computation(x: u32) -> u32 {
    codeview_annotation(&["string1", "string2", "string3"]);
    x + 1
}

// --- Error cases ---

fn non_const_arg(strs: &[&str]) {
    codeview_annotation(strs); //~ ERROR `codeview_annotation` expects a const array
    let s = "string1";
    codeview_annotation(&[s]); //~ ERROR `codeview_annotation` expects a const array
}

fn empty_array() {
    codeview_annotation(&[]); //~ ERROR `codeview_annotation` expects a non-empty array
}

fn wrong_type() {
    codeview_annotation(42); //~ ERROR mismatched types
}


fn main() {
    intrinsic_single();
    intrinsic_multiple();
    macro_single();
    macro_multiple();
    named_const_elements();
    mixed_named_const_and_literal_elements();
    named_const_slice();
    named_const_array_ref();
    let _ = annotated_computation(5);
    const _: u32 = annotated_computation(5);

    non_const_arg(&["a"]);
    empty_array();
    wrong_type();
}
