use serde::{Deserialize, Serialize};

/// Shared chart point for forecast and scenario views.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChartPoint {
    pub x: String,
    pub y: f32,
}

/// Line series used for point forecasts and scenarios.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LineSeries {
    pub label: String,
    pub points: Vec<ChartPoint>,
}

/// Interval band rendered around a forecast line.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntervalBand {
    pub label: String,
    pub lower: Vec<ChartPoint>,
    pub upper: Vec<ChartPoint>,
}

/// Regime timeline segment.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegimeSegment {
    pub start: String,
    pub end: String,
    pub label: String,
    pub probability: f32,
}

/// Chart model for the forecast dashboard.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ForecastChart {
    pub point: Option<LineSeries>,
    pub bands: Vec<IntervalBand>,
}

/// Chart model for scenario paths.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ScenarioChart {
    pub paths: Vec<LineSeries>,
    pub highlighted_path: Option<usize>,
}

/// Chart model for regimes and change points.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RegimeChart {
    pub segments: Vec<RegimeSegment>,
}

/// Single monitoring metric for compact operations dashboards.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricBar {
    pub label: String,
    pub value: f32,
    pub threshold: Option<f32>,
}

/// Chart model for health, evaluation, and alert summaries.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MonitoringChart {
    pub metrics: Vec<MetricBar>,
    pub alerts: Vec<String>,
}
