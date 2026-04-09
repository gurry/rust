//@ only-windows
//@ compile-flags: -C no-prepopulate-passes

#![crate_type = "lib"]
#![feature(codeview_annotation)]
#![feature(core_intrinsics)]

use std::intrinsics::codeview_annotation;

// CHECK-LABEL: @intrinsic_single_annotation
// CHECK: call void @llvm.codeview.annotation(metadata [[SINGLE:![0-9]+]])
#[no_mangle]
pub fn intrinsic_single_annotation() {
    codeview_annotation(&["test_annotation"]);
}

// CHECK-LABEL: @intrinsic_multiple_annotations
// CHECK: call void @llvm.codeview.annotation(metadata [[MULTI:![0-9]+]])
#[no_mangle]
pub fn intrinsic_multiple_annotations() {
    codeview_annotation(&["category", "subcategory", "details"]);
}

// CHECK-LABEL: @macro_single_annotation
// CHECK: call void @llvm.codeview.annotation(metadata [[MACRO_SINGLE:![0-9]+]])
#[no_mangle]
pub fn macro_single_annotation() {
    std::hint::codeview_annotation!("macro_test");
}

// CHECK-LABEL: @macro_multiple_annotations
// CHECK: call void @llvm.codeview.annotation(metadata [[MACRO_MULTI:![0-9]+]])
#[no_mangle]
pub fn macro_multiple_annotations() {
    std::hint::codeview_annotation!("Performance", "HotPath", "Critical");
}

const ANNOTATION_A: &str = "const_a";
const ANNOTATION_B: &str = "const_b";

// CHECK-LABEL: @named_const_elements
// CHECK: call void @llvm.codeview.annotation(metadata [[NAMED_CONST:![0-9]+]])
#[no_mangle]
pub fn named_const_elements() {
    codeview_annotation(&[ANNOTATION_A, ANNOTATION_B]);
}

// CHECK-LABEL: @mixed_const_and_literal
// CHECK: call void @llvm.codeview.annotation(metadata [[MIXED:![0-9]+]])
#[no_mangle]
pub fn mixed_const_and_literal() {
    codeview_annotation(&[ANNOTATION_A, "literal_mix"]);
}

const STRS_SLICE: &[&str] = &["slice_a", "slice_b"];

// CHECK-LABEL: @named_const_slice
// CHECK: call void @llvm.codeview.annotation(metadata [[CONST_SLICE:![0-9]+]])
#[no_mangle]
pub fn named_const_slice() {
    codeview_annotation(STRS_SLICE);
}

const STRS_ARRAY: [&str; 2] = ["arr_a", "arr_b"];

// CHECK-LABEL: @named_const_array_ref
// CHECK: call void @llvm.codeview.annotation(metadata [[CONST_ARRAY:![0-9]+]])
#[no_mangle]
pub fn named_const_array_ref() {
    codeview_annotation(&STRS_ARRAY);
}

// Metadata definitions are at the end of LLVM IR, so check them here
// CHECK-DAG: [[SINGLE]] = !{!"test_annotation"}
// CHECK-DAG: [[MULTI]] = !{!"category", !"subcategory", !"details"}
// CHECK-DAG: [[MACRO_SINGLE]] = !{!"macro_test"}
// CHECK-DAG: [[MACRO_MULTI]] = !{!"Performance", !"HotPath", !"Critical"}
// CHECK-DAG: [[NAMED_CONST]] = !{!"const_a", !"const_b"}
// CHECK-DAG: [[MIXED]] = !{!"const_a", !"literal_mix"}
// CHECK-DAG: [[CONST_SLICE]] = !{!"slice_a", !"slice_b"}
// CHECK-DAG: [[CONST_ARRAY]] = !{!"arr_a", !"arr_b"}
