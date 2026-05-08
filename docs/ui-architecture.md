# UI Architecture

The UI keeps API contracts, application state, chart view models, role permissions, and renderer-specific code separated. This allows dashboard behavior to be tested in native Rust while the `wasm32` Yew renderer owns browser integration.

Forecast state stores retrieval matches alongside decomposition, distribution, and imputed-history outputs. Monitoring state tracks reconciliation reports, evaluation reports, service status metrics, and alert summaries so hierarchy adjustments and post-deployment quality signals can be surfaced without coupling chart rendering to serving internals.
