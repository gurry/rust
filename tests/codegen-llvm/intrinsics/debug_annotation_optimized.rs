//@ only-windows
//@ compile-flags: -Copt-level=2
//
// Verify that debug annotations survive LLVM optimizations.
// The llvm.codeview.annotation intrinsic is marked noduplicate and
// writes to inaccessible memory, so LLVM must not remove it.

#![crate_type = "lib"]
#![feature(debug_annotation)]
#![feature(core_intrinsics)]

use std::intrinsics::debug_annotation;

// Verify that an annotation inside a function with other code is not
// removed even when surrounding code is optimized.
// CHECK-LABEL: @annotation_with_computation
// CHECK: call void @llvm.codeview.annotation(metadata [[COMP:![0-9]+]])
#[no_mangle]
pub fn annotation_with_computation(x: u32) -> u32 {
    debug_annotation(&["in_computation"]);
    x.wrapping_mul(31).wrapping_add(17)
}

// Verify that multiple annotations in the same function are all preserved.
// CHECK-LABEL: @multiple_annotations_same_fn
// CHECK: call void @llvm.codeview.annotation(metadata [[FIRST:![0-9]+]])
// CHECK: call void @llvm.codeview.annotation(metadata [[SECOND:![0-9]+]])
#[no_mangle]
pub fn multiple_annotations_same_fn() {
    debug_annotation(&["first"]);
    debug_annotation(&["second"]);
}

// Verify that annotations survive inlining: the annotation from
// the inlined callee should appear in the caller.
// CHECK-LABEL: @annotation_after_inlining
// CHECK: call void @llvm.codeview.annotation(metadata [[INLINED:![0-9]+]])
#[no_mangle]
pub fn annotation_after_inlining() {
    annotated_helper();
}

#[inline(always)]
fn annotated_helper() {
    debug_annotation(&["from_inlined_fn"]);
}

// CHECK-DAG: [[COMP]] = !{!"in_computation"}
// CHECK-DAG: [[FIRST]] = !{!"first"}
// CHECK-DAG: [[SECOND]] = !{!"second"}
// CHECK-DAG: [[INLINED]] = !{!"from_inlined_fn"}
