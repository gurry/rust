//@ only-windows
//@ compile-flags: -C no-prepopulate-passes

#![crate_type = "lib"]
#![feature(debug_annotation)]
#![feature(core_intrinsics)]

use std::intrinsics::debug_annotation;

// CHECK-LABEL: @intrinsic_single_annotation
// CHECK: call void @llvm.codeview.annotation(metadata [[SINGLE:![0-9]+]])
#[no_mangle]
pub fn intrinsic_single_annotation() {
    debug_annotation(&["test_annotation"]);
}

// CHECK-LABEL: @intrinsic_multiple_annotations
// CHECK: call void @llvm.codeview.annotation(metadata [[MULTI:![0-9]+]])
#[no_mangle]
pub fn intrinsic_multiple_annotations() {
    debug_annotation(&["category", "subcategory", "details"]);
}

// CHECK-LABEL: @macro_single_annotation
// CHECK: call void @llvm.codeview.annotation(metadata [[MACRO_SINGLE:![0-9]+]])
#[no_mangle]
pub fn macro_single_annotation() {
    std::hint::debug_annotation!("macro_test");
}

// CHECK-LABEL: @macro_multiple_annotations
// CHECK: call void @llvm.codeview.annotation(metadata [[MACRO_MULTI:![0-9]+]])
#[no_mangle]
pub fn macro_multiple_annotations() {
    std::hint::debug_annotation!("Performance", "HotPath", "Critical");
}

// Metadata definitions are at the end of LLVM IR, so check them here
// CHECK-DAG: [[SINGLE]] = !{!"test_annotation"}
// CHECK-DAG: [[MULTI]] = !{!"category", !"subcategory", !"details"}
// CHECK-DAG: [[MACRO_SINGLE]] = !{!"macro_test"}
// CHECK-DAG: [[MACRO_MULTI]] = !{!"Performance", !"HotPath", !"Critical"}
