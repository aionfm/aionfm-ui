#![cfg(target_arch = "wasm32")]

use crate::{AionFmUiState, PanelId};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(AionFmUiState::default);
    let active_panel = state.active_panel;
    html! {
        <div class="app-shell">
            <aside class="sidebar">
                <h1>{"AionFM"}</h1>
                <nav>
                    { for state.panels.iter().filter(|panel| panel.enabled).map(|panel| {
                        let label = panel.label.clone();
                        html! { <button type="button">{ label }</button> }
                    }) }
                </nav>
            </aside>
            <section class="workspace">
                <div class="toolbar">
                    <label for="entity">{"Entity"}</label>
                    <input id="entity" value={state.forecast.selected_entity.clone().unwrap_or_else(|| "store_42".into())} />
                    <label for="horizon">{"Horizon"}</label>
                    <input id="horizon" type="number" value={state.controls.horizon.to_string()} />
                </div>
                <div class="dashboard-grid">
                    <section class="panel">
                        <h2>{ panel_title(active_panel) }</h2>
                        <div class="chart-surface">{ "No forecast loaded" }</div>
                    </section>
                    <aside class="panel">
                        <h2>{"Metadata"}</h2>
                        <pre>{ serde_json::to_string_pretty(&state.metadata.attributes).unwrap_or_default() }</pre>
                    </aside>
                </div>
            </section>
        </div>
    }
}

fn panel_title(panel: PanelId) -> &'static str {
    match panel {
        PanelId::ForecastDashboard => "Forecast",
        PanelId::ScenarioExplorer => "Scenarios",
        PanelId::RegimeViewer => "Regimes",
        PanelId::MetadataPanel => "Metadata",
        PanelId::ControlPanel => "Controls",
        PanelId::MonitoringDashboard => "Monitoring",
    }
}

pub fn run() {
    yew::Renderer::<App>::new().render();
}
