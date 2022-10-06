extern crate itertools;
#[macro_use]
extern crate serde_derive;
// extern crate futures;
extern crate serde_json;
extern crate serde_qs as qs;

use crate::error::HeosError;

mod connection;
pub mod error;
mod types;
pub use connection::*;
pub use types::*;

pub type HeosResult<T> = Result<T, HeosError>;
