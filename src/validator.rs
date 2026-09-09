mod email;
#[doc(inline)]
pub use email::is_email;
mod url;
#[doc(inline)]
pub use url::is_url;
mod uuid;
#[doc(inline)]
pub use uuid::is_uuid;
mod ip;
#[doc(inline)]
pub use ip::{is_ip, is_ipv4, is_ipv6};
mod datetime;
#[doc(inline)]
pub use datetime::is_datetime;



