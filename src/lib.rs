#![feature(portable_simd)]
#![feature(test)]
#![feature(generators, generator_trait)]
#![feature(unboxed_closures, fn_traits)]
#![feature(type_name_of_val)]
extern crate test;
#[cfg(test)]
mod actix_web;
#[cfg(test)]
mod async_;
mod bincode;
#[cfg(test)]
mod count_receipts_csv_total_amount;
#[cfg(test)]
mod error_handling;
mod simd;
mod type_system;
