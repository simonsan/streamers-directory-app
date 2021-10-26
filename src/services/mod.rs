pub mod aoc_ref;
pub mod requests;

pub use requests::{
    get_token,
    is_authenticated,
    limit,
    set_token,
    Requests,
};
