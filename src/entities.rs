mod announcements;
mod asset_dsc;
mod auth_data;
mod bucket;
mod status;
mod user_bucket;
mod user_buckets;
mod user_cash;

use crate::parsable::Parsable;
use crate::parse::Parser;
pub use announcements::Announcements;
pub use auth_data::AuthData;
pub use bucket::Bucket;
pub use user_bucket::UserBucket;
pub use user_buckets::UserBuckets;
pub use user_cash::UserCash;

pub fn just_parse<T>(input: &str) -> Result<(&str, T), ()>
where
    T: Parsable,
{
    T::parser().parse(input)
}
