use crate::metrics::Metric;

pub fn collect() -> Metric {
    Metric::Gauge {
        name: "is_alive".to_string(),
        value: 1.0,
    }
}
