use nalgebra::{RealField, SMatrix};
use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, Mul, Sub},
};

pub trait Real: Copy + RealField {
    fn to_f64(self) -> f64;
}

impl Real for f32 {
    fn to_f64(self) -> f64 {
        f64::from(self)
    }
}

impl Real for f64 {
    fn to_f64(self) -> f64 {
        self
    }
}

pub trait State<T>:
    Clone
    + Copy
    + Debug
    + Send
    + Sync
    + Display
    + Add<Output = Self>
    + Sub<Output = Self>
    + AddAssign
    + Mul<T, Output = Self>
    + Div<T, Output = Self>
{
    fn len(&self) -> usize;
    fn zeros() -> Self;
    fn flat(&self) -> String;
}

impl<T: Real> State<T> for T {
    fn len(&self) -> usize {
        1
    }

    fn zeros() -> Self {
        T::zero()
    }

    fn flat(&self) -> String {
        self.to_string()
    }
}

impl<T, const R: usize, const C: usize> State<T> for SMatrix<T, R, C>
where
    T: Real,
{
    fn len(&self) -> usize {
        R * C
    }

    fn zeros() -> Self {
        SMatrix::<T, R, C>::zeros()
    }

    fn flat(&self) -> String {
        self.as_slice()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    }
}
