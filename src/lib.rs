//!
//! This crate provides a way to easily use and work with signatures.
//!
//! *Signatures* are a pattern of bytes, usually code, with optional placeholders.
//! This is used for patching binaries that might change via updates.
//!
//! The crate supports what I've coined *static* and *dynamic* signatures.
//!
//! - *Dynamic signatures* hold the pattern at run-time, and are generally preferred.
//! - *Static signatures* evaluate the pattern at compile-time, and store an offset to a module base at run-time. This WILL break every update, and will need a recompilation.
//!

pub mod dynamic_signature;
pub mod static_signature;

pub extern crate signature_macro;

/// This trait simply defines a way to get the address of a pattern in memory, at run-time.
pub trait Signature {
    fn get(&mut self) -> Option<*mut u8>;
}
