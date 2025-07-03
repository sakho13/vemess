//! Core library for quantum chemical calculations.
//!
//! This crate provides fundamental data structures such as `Molecule`
//! and `BasisSet` used by higher level computation modules.

pub mod error;
pub mod molecule;
pub mod basis;
pub mod units;

pub use molecule::{Atom, Molecule};
pub use basis::{BasisSet, ContractedGaussian, PrimitiveGaussian};
pub use error::CoreError;
