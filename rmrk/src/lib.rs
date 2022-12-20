#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

pub mod collection;
pub mod errors;
pub mod minting;
pub mod multiasset;
pub mod nesting;
pub mod traits;
pub mod types;

mod rmrk;
pub use rmrk::*;
