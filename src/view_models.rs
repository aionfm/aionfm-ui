use crate::{
    charts::{
        ChartPoint, ForecastChart, IntervalBand, LineSeries, RegimeChart, RegimeSegment,
        ScenarioChart,
    },
    state::AionFmUiState,
};
use aionfm_utils::{EntityForecast, ForecastResponse};

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

/// Applies a forecast response to the root UI state.
pub fn apply_forecast_response(state: &mut AionFmUiState, response: ForecastResponse) {
    if let Some(first) = response.results.first() {
        state.forecast.selected_entity = Some(first.entity_id.clone());
        state.forecast.selected_target = Some(first.target.clone());
        state.forecast.chart = forecast_chart(first);
        state.scenarios.chart = scenario_chart(first);
        state.regimes.chart = regime_chart(first);
        state.metadata.entity_id = Some(first.entity_id.clone());
        state.metadata.attributes = first.metadata.clone();
    }
    state.forecast.last_response = Some(response);
    state.loading = false;
    state.error_message = None;
}

#[cfg(test)]
mod tests {
    use super::*;
    use aionfm_utils::EntityForecast;
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
            scenario_paths: None,
            regime_probabilities: None,
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
            scenario_paths: Some(vec![vec![1.0, 2.0]]),
            regime_probabilities: Some(BTreeMap::from([("stable".into(), 1.0)])),
            explanation: None,
            metadata: BTreeMap::new(),
        };
        let mut state = AionFmUiState::default();
        apply_forecast_response(
            &mut state,
            ForecastResponse::new("AionFM", "test", vec![entity]),
        );
        assert_eq!(state.forecast.selected_entity.as_deref(), Some("entity"));
        assert_eq!(state.scenarios.chart.paths.len(), 1);
        assert_eq!(state.regimes.chart.segments.len(), 1);
    }
}
