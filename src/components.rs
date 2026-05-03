use serde::{Deserialize, Serialize};

/// Top-level panels in the AionFM UI.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PanelId {
    ForecastDashboard,
    ScenarioExplorer,
    RegimeViewer,
    MetadataPanel,
    ControlPanel,
    MonitoringDashboard,
}

/// Navigation descriptor for a UI panel.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PanelDescriptor {
    pub id: PanelId,
    pub label: String,
    pub enabled: bool,
}

/// Request controls that map to API forecast options.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ControlPanelState {
    pub horizon: usize,
    pub scenario_count: usize,
    pub return_regimes: bool,
    pub return_scenarios: bool,
    pub enforce_constraints: bool,
    pub use_retrieval: bool,
}

impl Default for ControlPanelState {
    fn default() -> Self {
        Self {
            horizon: 30,
            scenario_count: 20,
            return_regimes: true,
            return_scenarios: true,
            enforce_constraints: false,
            use_retrieval: false,
        }
    }
}
