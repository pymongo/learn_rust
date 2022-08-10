#![feature(portable_simd)]
#![feature(test)]
#![feature(generators, generator_trait)]
#![feature(unboxed_closures, fn_traits)]
#![feature(type_name_of_val)]
#![cfg(test)]
extern crate test;
mod actix_web;
mod async_;
mod bincode;
mod count_receipts_csv_total_amount;
mod error_handling;
mod simd;
mod type_system;
mod zip_tar_gzip;
