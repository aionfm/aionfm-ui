# AionFM UI

Rust/WASM UI shell for forecast dashboards, scenario exploration, regime inspection, metadata panels, controls, and monitoring views from `aionfm-spec` Doc58.

The pure Rust modules compile on the host and model UI state independently from the web renderer. The `wasm32` renderer is gated behind target-specific dependencies and can be served with Trunk once the WebAssembly target is installed.

## Commands

```sh
cargo fmt
cargo check
cargo test
rustup target add wasm32-unknown-unknown
trunk serve
```

## Structure

- `src/state.rs`: application state and dashboard selections.
- `src/charts.rs`: chart view models for forecast bands, scenarios, regimes, and monitoring.
- `src/components.rs`: panel descriptors and control state.
- `src/api.rs`: typed API service layer configuration.
- `src/roles.rs`: analyst, planner, operator, and administrator permissions.
- `src/web.rs`: Yew renderer for `wasm32`.
