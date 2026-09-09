
mod validate;
#[doc(inline)]
pub use validate::Validate;

pub mod validator;

pub mod error;
#[doc(inline)]
pub use error::*;

mod input;
pub(crate) use input::ZInput;

pub mod schema;
#[doc(inline)]
pub use schema::*;

pub mod prelude {
    pub use crate::*;
}
