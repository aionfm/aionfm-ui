# UI Architecture

The UI keeps API contracts, application state, chart view models, role permissions, and renderer-specific code separated. This allows dashboard behavior to be tested in native Rust while the `wasm32` Yew renderer owns browser integration.
