use aionfm_utils::{
    AdaptationRequest, BatchForecastRequest, ForecastResponse, InterpretationRequest,
    ModelDescriptor, ScenarioRequest, ServiceStatus,
};
use serde::{Deserialize, Serialize};

/// Browser API configuration.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct UiApiConfig {
    pub base_url: String,
    #[serde(default)]
    pub api_key: Option<String>,
}

impl Default for UiApiConfig {
    fn default() -> Self {
        Self {
            base_url: "/".into(),
            api_key: None,
        }
    }
}

impl UiApiConfig {
    pub fn endpoint(&self, path: &str) -> String {
        format!(
            "{}/{}",
            self.base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        )
    }
}

/// UI service layer contract. The web renderer can implement this with fetch/gloo.
pub trait UiApiService {
    type Error;

    fn forecast(
        &self,
        request: BatchForecastRequest,
    ) -> impl std::future::Future<Output = Result<ForecastResponse, Self::Error>>;
    fn scenario(
        &self,
        request: ScenarioRequest,
    ) -> impl std::future::Future<Output = Result<ForecastResponse, Self::Error>>;
    fn interpretation(
        &self,
        request: InterpretationRequest,
    ) -> impl std::future::Future<Output = Result<ForecastResponse, Self::Error>>;
    fn models(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<ModelDescriptor>, Self::Error>>;
    fn status(&self) -> impl std::future::Future<Output = Result<ServiceStatus, Self::Error>>;
    fn adapt(
        &self,
        request: AdaptationRequest,
    ) -> impl std::future::Future<Output = Result<aionfm_utils::AdaptationStatus, Self::Error>>;
}
