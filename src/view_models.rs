use crate::{
    charts::{
        ChartPoint, ForecastChart, IntervalBand, LineSeries, MetricBar, MonitoringChart,
        RegimeChart, RegimeSegment, ScenarioChart,
    },
    state::AionFmUiState,
};
use aionfm_utils::{EntityForecast, EvaluationReport, ForecastResponse, ServiceStatus};

/// Converts schema forecasts into chart view models.
pub fn forecast_chart(entity: &EntityForecast) -> ForecastChart {
    let point = LineSeries {
        label: "Point".into(),
        points: entity
            .point_forecast
            .iter()
            .enumerate()
            .map(|(index, value)| ChartPoint {
                x: index.to_string(),
                y: *value,
            })
            .collect(),
    };
    let bands = entity
        .prediction_intervals
        .iter()
        .map(|(label, interval)| IntervalBand {
            label: label.clone(),
            lower: interval
                .lower
                .iter()
                .enumerate()
                .map(|(index, value)| ChartPoint {
                    x: index.to_string(),
                    y: *value,
                })
                .collect(),
            upper: interval
                .upper
                .iter()
                .enumerate()
                .map(|(index, value)| ChartPoint {
                    x: index.to_string(),
                    y: *value,
                })
                .collect(),
        })
        .collect();
    ForecastChart {
        point: Some(point),
        bands,
    }
}

pub fn scenario_chart(entity: &EntityForecast) -> ScenarioChart {
    let paths = entity
        .scenario_paths
        .clone()
        .unwrap_or_default()
        .into_iter()
        .enumerate()
        .map(|(scenario_index, values)| LineSeries {
            label: format!("Scenario {}", scenario_index + 1),
            points: values
                .iter()
                .enumerate()
                .map(|(index, value)| ChartPoint {
                    x: index.to_string(),
                    y: *value,
                })
                .collect(),
        })
        .collect();
    ScenarioChart {
        paths,
        highlighted_path: None,
    }
}

pub fn regime_chart(entity: &EntityForecast) -> RegimeChart {
    let mut cursor = 0;
    let segments = entity
        .regime_probabilities
        .clone()
        .unwrap_or_default()
        .into_iter()
        .map(|(label, probability)| {
            let width = ((probability * entity.forecast_horizon as f32).ceil() as usize).max(1);
            let start = cursor;
            cursor = (cursor + width).min(entity.forecast_horizon);
            RegimeSegment {
                start: start.to_string(),
                end: cursor.to_string(),
                label,
                probability,
            }
        })
        .collect();
    RegimeChart { segments }
}

pub fn monitoring_chart(
    status: Option<&ServiceStatus>,
    report: Option<&EvaluationReport>,
) -> MonitoringChart {
    let mut metrics = Vec::new();
    let mut alerts = Vec::new();
    if let Some(status) = status {
        for key in [
            "request_count",
            "evaluation_count",
            "average_interval_width",
            "quantile_crossing_rate",
            "last_mae",
            "last_smape",
        ] {
            if let Some(value) = status.metrics.get(key) {
                metrics.push(MetricBar {
                    label: key.into(),
                    value: *value,
                    threshold: monitoring_threshold(key),
                });
            }
        }
        alerts.extend(status.alerts.iter().map(|alert| alert.message.clone()));
    }
    if let Some(report) = report {
        for key in [
            "overall_mae",
            "overall_rmse",
            "overall_smape",
            "overall_wape",
            "average_quantile_calibration_error",
        ] {
            if let Some(value) = report.metrics.get(key) {
                metrics.push(MetricBar {
                    label: key.into(),
                    value: *value,
                    threshold: monitoring_threshold(key),
                });
            }
        }
        alerts.extend(report.alerts.iter().map(|alert| alert.message.clone()));
    }
    MonitoringChart { metrics, alerts }
}

/// Applies a forecast response to the root UI state.
pub fn apply_forecast_response(state: &mut AionFmUiState, response: ForecastResponse) {
    let reconciliation_report = response.reconciliation_report.clone();
    if let Some(first) = response.results.first() {
        state.forecast.selected_entity = Some(first.entity_id.clone());
        state.forecast.selected_target = Some(first.target.clone());
        state.forecast.chart = forecast_chart(first);
        state.forecast.decomposition = first.decomposition.clone();
        state.forecast.distribution = first.distribution.clone();
        state.forecast.imputed_history = first.imputed_history.clone();
        state.forecast.retrieval_matches = first.retrieval_matches.clone().unwrap_or_default();
        state.scenarios.chart = scenario_chart(first);
        state.regimes.chart = regime_chart(first);
        state.metadata.entity_id = Some(first.entity_id.clone());
        state.metadata.attributes = first.metadata.clone();
    }
    state.forecast.last_response = Some(response);
    state.monitoring.reconciliation_report = reconciliation_report;
    state.loading = false;
    state.error_message = None;
}

pub fn apply_service_status(state: &mut AionFmUiState, status: ServiceStatus) {
    state.monitoring.service_status = Some(status);
    state.monitoring.chart = monitoring_chart(
        state.monitoring.service_status.as_ref(),
        state.monitoring.evaluation_report.as_ref(),
    );
    state.loading = false;
    state.error_message = None;
}

pub fn apply_evaluation_report(state: &mut AionFmUiState, report: EvaluationReport) {
    state.monitoring.evaluation_report = Some(report);
    state.monitoring.chart = monitoring_chart(
        state.monitoring.service_status.as_ref(),
        state.monitoring.evaluation_report.as_ref(),
    );
    state.loading = false;
    state.error_message = None;
}

fn monitoring_threshold(key: &str) -> Option<f32> {
    match key {
        "overall_smape" | "last_smape" => Some(0.25),
        "overall_wape" => Some(0.25),
        "average_quantile_calibration_error" => Some(0.10),
        "quantile_crossing_rate" => Some(0.0),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aionfm_utils::{
        EntityForecast, EvaluationReport, ReconciliationReport, RetrievalMatch, ServiceStatus,
    };
    use std::collections::BTreeMap;

    #[test]
    fn builds_point_series() {
        let chart = forecast_chart(&EntityForecast {
            entity_id: "entity".into(),
            forecast_horizon: 2,
            frequency: "D".into(),
            target: "value".into(),
            point_forecast: vec![1.0, 2.0],
            quantiles: BTreeMap::new(),
            prediction_intervals: BTreeMap::new(),
            decomposition: None,
            distribution: None,
            imputed_history: None,
            scenario_paths: None,
            regime_probabilities: None,
            regime_timeline: None,
            constraint_report: None,
            retrieval_matches: None,
            explanation: None,
            metadata: BTreeMap::new(),
        });
        assert_eq!(chart.point.unwrap().points.len(), 2);
    }

    #[test]
    fn applies_response_to_state() {
        let entity = EntityForecast {
            entity_id: "entity".into(),
            forecast_horizon: 2,
            frequency: "D".into(),
            target: "value".into(),
            point_forecast: vec![1.0, 2.0],
            quantiles: BTreeMap::new(),
            prediction_intervals: BTreeMap::new(),
            decomposition: None,
            distribution: None,
            imputed_history: None,
            scenario_paths: Some(vec![vec![1.0, 2.0]]),
            regime_probabilities: Some(BTreeMap::from([("stable".into(), 1.0)])),
            regime_timeline: None,
            constraint_report: None,
            retrieval_matches: Some(vec![RetrievalMatch {
                source_entity_id: "entity".into(),
                start_index: 0,
                window_len: 2,
                similarity: 1.0,
                regime_label: Some("stable".into()),
                outcome_preview: vec![1.0, 2.0],
            }]),
            explanation: None,
            metadata: BTreeMap::new(),
        };
        let mut state = AionFmUiState::default();
        let mut response = ForecastResponse::new("AionFM", "test", vec![entity]);
        response.reconciliation_report = Some(ReconciliationReport {
            adjusted_entities: vec!["entity".into()],
            ..Default::default()
        });
        apply_forecast_response(&mut state, response);
        assert_eq!(state.forecast.selected_entity.as_deref(), Some("entity"));
        assert_eq!(state.scenarios.chart.paths.len(), 1);
        assert_eq!(state.regimes.chart.segments.len(), 1);
        assert_eq!(state.forecast.retrieval_matches.len(), 1);
        assert!(state.monitoring.reconciliation_report.is_some());
    }

    #[test]
    fn applies_monitoring_reports_to_chart() {
        let mut state = AionFmUiState::default();
        apply_service_status(
            &mut state,
            ServiceStatus {
                status: "healthy".into(),
                version: "test".into(),
                model_loaded: true,
                queue_depth: 0,
                p50_latency_ms: Some(10.0),
                p95_latency_ms: Some(15.0),
                metrics: BTreeMap::from([("request_count".into(), 2.0)]),
                alerts: vec![],
            },
        );
        apply_evaluation_report(
            &mut state,
            EvaluationReport::new(
                "AionFM",
                "test",
                vec![],
                BTreeMap::from([("overall_mae".into(), 1.0)]),
                vec![],
            ),
        );
        assert!(state
            .monitoring
            .chart
            .metrics
            .iter()
            .any(|metric| metric.label == "overall_mae"));
    }
}
