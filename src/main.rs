#![recursion_limit = "2048"]

use std::env;

#[macro_use]
mod component;
mod models;
mod route;
mod string;

use crate::component::root::RootComponent;

pub const THOTH_GRAPHQL_API: &str = env!("THOTH_GRAPHQL_API");
pub const THOTH_EXPORT_API: &str = env!("THOTH_EXPORT_API");
/// Default number of milliseconds to wait before sending a search query
const DEFAULT_DEBOUNCING_TIMEOUT: u32 = 500;

pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::start_app::<RootComponent>();
}
