#![doc(test(no_crate_inject))]
#![allow(unused_imports)]
// #![allow(incomplete_features)]
// #![feature(const_generics)]
// #![feature(const_evaluatable_checked)]
#![doc(html_playground_url = "https://rgeometry.org/rgeometry-playground/")]
#![doc(test(no_crate_inject))]
use num_traits::*;
use std::iter::Sum;
use std::ops::*;

pub mod algorithms;
mod array;
pub mod data;
mod intersection;
mod matrix;
mod transformation;

pub use array::Orientation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
  InsufficientVertices,
  SelfIntersections,
  DuplicatePoints,
  /// Two consecutive line segments are either colinear or oriented clockwise.
  ConvexViolation,
  ClockWiseViolation,
}
pub trait PolygonScalar<T = Self, Output = Self>:
  PolygonScalarRef<T, Output>
  + AddAssign<Output>
  + MulAssign<Output>
  + FromPrimitive
  + One
  + Zero
  + Sum
  + Ord
  + Neg<Output = Output>
{
}
impl<T, Rhs, Output> PolygonScalar<Rhs, Output> for T where
  T: PolygonScalarRef<Rhs, Output>
    + AddAssign<Output>
    + MulAssign<Output>
    + FromPrimitive
    + One
    + Zero
    + Sum
    + Ord
    + Neg<Output = Output>
{
}

pub trait PolygonScalarRef<T = Self, Output = Self>: Clone + NumOps<T, Output> {}
impl<T, Rhs, Output> PolygonScalarRef<Rhs, Output> for T where T: Clone + NumOps<Rhs, Output> {}
