#![allow(clippy::too_many_arguments)]
#![allow(clippy::missing_safety_doc)]
#![allow(unexpected_cfgs)] // TODO: Upgrade rkyv to 0.8

pub mod base;
pub mod body;
pub mod character;
pub mod consts;
pub mod error;
pub mod shape;
pub mod system;
pub mod vtable;

pub use base::*;
pub use body::*;
pub use character::*;
pub use consts::*;
pub use error::*;
pub use jolt_macros::vdata;
pub use shape::*;
pub use system::*;
pub use vtable::*;

#[cfg(all(windows, feature = "debug-renderer"))]
pub mod debug;

#[cfg(test)]
mod test_callback;
