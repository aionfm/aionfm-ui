use crate::{
    charts::{ForecastChart, RegimeChart, ScenarioChart},
    components::{ControlPanelState, PanelDescriptor, PanelId},
    roles::UserRole,
};
use aionfm_utils::{
    DistributionForecast, ForecastDecomposition, ForecastResponse, Metadata, ModelDescriptor,
    ServiceStatus,
};
use serde::{Deserialize, Serialize};

/// Forecast dashboard selections and rendered chart model.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ForecastDashboardState {
    pub selected_entity: Option<String>,
    pub selected_target: Option<String>,
    pub chart: ForecastChart,
    pub decomposition: Option<ForecastDecomposition>,
    pub distribution: Option<DistributionForecast>,
    pub imputed_history: Option<Vec<f32>>,
    pub last_response: Option<ForecastResponse>,
}

/// Scenario explorer state.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ScenarioExplorerState {
    pub chart: ScenarioChart,
    pub selected_scenario_type: Option<String>,
}

/// Regime viewer state.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RegimeViewerState {
    pub chart: RegimeChart,
    pub selected_regime: Option<String>,
}

/// Metadata side panel state.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MetadataPanelState {
    pub entity_id: Option<String>,
    pub attributes: Metadata,
}

/// Monitoring dashboard state.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MonitoringState {
    pub service_status: Option<ServiceStatus>,
    pub models: Vec<ModelDescriptor>,
}

/// Root UI state for dashboards and operations views.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AionFmUiState {
    pub role: UserRole,
    pub active_panel: PanelId,
    pub panels: Vec<PanelDescriptor>,
    pub controls: ControlPanelState,
    pub forecast: ForecastDashboardState,
    pub scenarios: ScenarioExplorerState,
    pub regimes: RegimeViewerState,
    pub metadata: MetadataPanelState,
    pub monitoring: MonitoringState,
    pub loading: bool,
    pub error_message: Option<String>,
}

impl Default for AionFmUiState {
    fn default() -> Self {
        let role = UserRole::Analyst;
        Self {
            role,
            active_panel: PanelId::ForecastDashboard,
            panels: panels_for_role(role),
            controls: ControlPanelState::default(),
            forecast: ForecastDashboardState::default(),
            scenarios: ScenarioExplorerState::default(),
            regimes: RegimeViewerState::default(),
            metadata: MetadataPanelState::default(),
            monitoring: MonitoringState::default(),
            loading: false,
            error_message: None,
        }
    }
}

pub fn panels_for_role(role: UserRole) -> Vec<PanelDescriptor> {
    let permissions = role.permissions();
    vec![
        PanelDescriptor {
            id: PanelId::ForecastDashboard,
            label: "Forecast".into(),
            enabled: permissions.view_forecasts,
        },
        PanelDescriptor {
            id: PanelId::ScenarioExplorer,
            label: "Scenarios".into(),
            enabled: permissions.generate_scenarios,
        },
        PanelDescriptor {
            id: PanelId::RegimeViewer,
            label: "Regimes".into(),
            enabled: permissions.view_forecasts,
        },
        PanelDescriptor {
            id: PanelId::MetadataPanel,
            label: "Metadata".into(),
            enabled: permissions.view_forecasts,
        },
        PanelDescriptor {
            id: PanelId::ControlPanel,
            label: "Controls".into(),
            enabled: permissions.generate_scenarios,
        },
        PanelDescriptor {
            id: PanelId::MonitoringDashboard,
            label: "Monitoring".into(),
            enabled: permissions.monitor_models,
        },
    ]
}
