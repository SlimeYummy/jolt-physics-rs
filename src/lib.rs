mod base;
mod character;
mod layer;
mod shape;
mod system;

pub use base::{BodyID, IndexedTriangle, RefPhysicsMaterial, RefPhysicsSystem, RefShape};
pub use character::*;
pub use layer::*;
pub use shape::*;
pub use system::*;

#[cfg(all(windows, feature = "debug-renderer"))]
mod debug;

#[cfg(all(windows, feature = "debug-renderer"))]
pub use debug::*;
