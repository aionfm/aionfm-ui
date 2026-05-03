//! Rust/WASM UI shell for AionFM.

pub mod api;
pub mod charts;
pub mod components;
pub mod roles;
pub mod state;
pub mod view_models;

#[cfg(target_arch = "wasm32")]
pub mod web;

pub use api::*;
pub use charts::*;
pub use components::*;
pub use roles::*;
pub use state::*;
pub use view_models::*;
