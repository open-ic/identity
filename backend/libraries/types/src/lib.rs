use candid::Principal;

mod app;
mod attribute_id;
mod error;
mod indexed_event;
mod phone_number;
mod verification_code;

pub use app::*;
pub use attribute_id::*;
pub use error::*;
pub use indexed_event::*;
pub use phone_number::*;
pub use verification_code::*;

pub type CanisterId = Principal;
pub type Milliseconds = u64;
pub type TimestampMillis = u64;
pub type TimestampNanos = u64;
