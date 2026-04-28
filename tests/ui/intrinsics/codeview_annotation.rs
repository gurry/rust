// Verifies codeview_annotation compile-time behavior:
// - Happy paths: inline literals, macro usage, named const elements,
//   mixed const/literal, const slices, const array refs.
// - Error cases: wrong types, non-const usage in const fn.
// - Non-const arguments compile but are silently ignored at codegen time.
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

const STR_A: &str = "string1";
const STR_B: &str = "string2";
const STR_C: &str = "string3";
fn named_const_elements() {
    codeview_annotation(&[STR_A, STR_B, STR_C]);
}

fn mixed_named_const_and_literal_elements() {
    codeview_annotation(&[STR_A, "string2", "string3"]);
}

const STRS_SLICE: &[&str] = &["string1", "string2", "string3"];
fn named_const_slice() {
    codeview_annotation(STRS_SLICE);
}

const STRS_ARRAY: [&str; 3] = ["string1", "string2", "string3"];
fn named_const_array_ref() {
    codeview_annotation(&STRS_ARRAY);
}

// Use in const function is not currently supported because the intrinsic
// does not have a const-evaluable body.
const fn const_func(x: u32) -> u32 {
    codeview_annotation(&["string1", "string2", "string3"]);
    //~^ ERROR cannot call non-const intrinsic `codeview_annotation` in constant functions
    x + 1
}

// Consts having generic params
trait Var {
    const NAME: &str;
    const VAL: &str;
}

impl Var for i32 {
    const NAME: &str = "i32";
    const VAL: &str = "5";
}

fn generic_const_elements<T: Var>() {
    codeview_annotation(&["string", T::NAME, T::VAL]);
}

// Non-const arguments are accepted at compile time but the codegen
// MIR walker will silently produce no annotation for them.
fn non_const_arg(strs: &[&str]) {
    codeview_annotation(strs);
    let s = "string1";
    codeview_annotation(&[s]);
}

fn empty_array() {
    codeview_annotation(&[]);
}

fn wrong_type() {
    codeview_annotation(42); //~ ERROR mismatched types
}

// Generic associated consts are now accepted (the MIR walker
// extracts strings at codegen time when the type is monomorphized).
trait HasStrs {
    const STRS: &[&str];
}

impl HasStrs for i32 {
    const STRS: &[&str] = &["string1", "string2", "string3"];
}

fn generic_associated_const_slice<T: HasStrs>() {
    codeview_annotation(T::STRS);
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
    let _ = const_func(5);
    generic_const_elements::<i32>();
    non_const_arg(&["a"]);
    empty_array();
    wrong_type();
    generic_associated_const_slice::<i32>();
}
