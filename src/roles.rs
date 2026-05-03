use serde::{Deserialize, Serialize};

/// UI roles from the product specification.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Analyst,
    Planner,
    Operator,
    Administrator,
}

impl Default for UserRole {
    fn default() -> Self {
        Self::Analyst
    }
}

/// Capability matrix used by navigation and controls.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Permissions {
    pub view_forecasts: bool,
    pub generate_scenarios: bool,
    pub monitor_models: bool,
    pub manage_models: bool,
}

impl UserRole {
    pub fn permissions(self) -> Permissions {
        match self {
            Self::Analyst => Permissions {
                view_forecasts: true,
                generate_scenarios: false,
                monitor_models: false,
                manage_models: false,
            },
            Self::Planner => Permissions {
                view_forecasts: true,
                generate_scenarios: true,
                monitor_models: false,
                manage_models: false,
            },
            Self::Operator => Permissions {
                view_forecasts: true,
                generate_scenarios: true,
                monitor_models: true,
                manage_models: false,
            },
            Self::Administrator => Permissions {
                view_forecasts: true,
                generate_scenarios: true,
                monitor_models: true,
                manage_models: true,
            },
        }
    }
}
