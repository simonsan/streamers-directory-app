#![recursion_limit = "1024"]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::eval_order_dependence)]

pub mod app;
pub mod components;
pub mod routes;
pub mod services;
pub mod types;
pub mod error;

use app::App;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
