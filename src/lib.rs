#![feature(portable_simd)]
#![feature(test)]
#![feature(generators, generator_trait)]
#![feature(unboxed_closures, fn_traits)]
extern crate test;
#[cfg(test)]
mod actix_web;
#[cfg(test)]
mod async_;
mod simd;
mod type_system;
#[cfg(test)]
mod error_handling;