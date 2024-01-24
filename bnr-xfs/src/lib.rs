pub(crate) mod arrays;
mod callback_response;
mod capabilities;
mod cash_unit;
mod currency;
mod device_handle;
mod dispense;
mod error;
mod intermediate_event;
#[macro_use]
mod macros;
mod status;
pub mod xfs;

pub use callback_response::*;
pub use capabilities::*;
pub use cash_unit::*;
pub use currency::*;
pub use device_handle::*;
pub use dispense::*;
pub use error::*;
pub use intermediate_event::*;
pub use status::*;

create_xfs_i4!(Size, "size", "Represents the size of a list.");
create_xfs_i4!(MaxSize, "maxSize", "Represents the maximum size of a list.");
