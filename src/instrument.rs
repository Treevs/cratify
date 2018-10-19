#![feature(custom_attribute)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel;

pub(crate) mod db;

use crate::db::models::*;

fn main() -> Result<(), Box<std::error::Error>> {
    println!("Hello world");
    Ok(())
}
