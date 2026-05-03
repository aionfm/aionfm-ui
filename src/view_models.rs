use crate::charts::{ChartPoint, ForecastChart, IntervalBand, LineSeries, ScenarioChart};
use aionfm_utils::EntityForecast;

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
}
