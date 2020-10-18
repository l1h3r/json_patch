#[macro_use]
extern crate serde;

mod error;
mod merge;
mod operation;
mod patch;
mod traits;

pub use self::error::Error;
pub use self::error::Result;

pub use self::merge::merge_mut;
pub use self::merge::merge_ref;

pub use self::operation::Operation;

pub use self::patch::Patch;

pub use self::traits::CanPatch;
