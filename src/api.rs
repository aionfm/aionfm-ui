use aionfm_utils::{
    AdaptationRequest, BatchForecastRequest, ForecastResponse, InterpretationRequest,
    ModelDescriptor, ScenarioRequest, ServiceStatus,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

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

/// UI API errors for browser and test service implementations.
#[derive(Debug, Error)]
pub enum UiApiError {
    #[error("network error: {0}")]
    Network(String),
    #[error("serialization error: {0}")]
    Serialization(String),
    #[error("api error: status {status}: {message}")]
    Api { status: u16, message: String },
}

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Debug)]
pub struct WasmApiService {
    config: UiApiConfig,
}

#[cfg(target_arch = "wasm32")]
impl WasmApiService {
    pub fn new(config: UiApiConfig) -> Self {
        Self { config }
    }

    async fn get_json<R>(&self, path: &str) -> Result<R, UiApiError>
    where
        R: for<'de> Deserialize<'de>,
    {
        let mut request = gloo_net::http::Request::get(&self.config.endpoint(path));
        if let Some(api_key) = &self.config.api_key {
            request = request.header("x-api-key", api_key);
        }
        decode_response(request.send().await).await
    }

    async fn post_json<B, R>(&self, path: &str, body: &B) -> Result<R, UiApiError>
    where
        B: Serialize,
        R: for<'de> Deserialize<'de>,
    {
        let mut request = gloo_net::http::Request::post(&self.config.endpoint(path));
        if let Some(api_key) = &self.config.api_key {
            request = request.header("x-api-key", api_key);
        }
        let request = request
            .json(body)
            .map_err(|error| UiApiError::Serialization(error.to_string()))?;
        decode_response(request.send().await).await
    }
}

#[cfg(target_arch = "wasm32")]
impl UiApiService for WasmApiService {
    type Error = UiApiError;

    async fn forecast(
        &self,
        request: BatchForecastRequest,
    ) -> Result<ForecastResponse, Self::Error> {
        self.post_json("/v1/forecast", &request).await
    }

    async fn scenario(&self, request: ScenarioRequest) -> Result<ForecastResponse, Self::Error> {
        self.post_json("/v1/scenario", &request).await
    }

    async fn interpretation(
        &self,
        request: InterpretationRequest,
    ) -> Result<ForecastResponse, Self::Error> {
        self.post_json("/v1/interpretation", &request).await
    }

    async fn models(&self) -> Result<Vec<ModelDescriptor>, Self::Error> {
        self.get_json("/v1/models").await
    }

    async fn status(&self) -> Result<ServiceStatus, Self::Error> {
        self.get_json("/v1/status").await
    }

    async fn adapt(
        &self,
        request: AdaptationRequest,
    ) -> Result<aionfm_utils::AdaptationStatus, Self::Error> {
        self.post_json("/v1/adapt", &request).await
    }
}

#[cfg(target_arch = "wasm32")]
async fn decode_response<R>(
    response: Result<gloo_net::http::Response, gloo_net::Error>,
) -> Result<R, UiApiError>
where
    R: for<'de> Deserialize<'de>,
{
    let response = response.map_err(|error| UiApiError::Network(error.to_string()))?;
    let status = response.status();
    if (200..300).contains(&status) {
        response
            .json::<R>()
            .await
            .map_err(|error| UiApiError::Serialization(error.to_string()))
    } else {
        let message = response
            .text()
            .await
            .unwrap_or_else(|_| "unable to read error body".into());
        Err(UiApiError::Api { status, message })
    }
}
