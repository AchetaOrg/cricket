use sysinfo::{System, SystemExt};
use crate::metrics::Metric;
use sysinfo::CpuExt;

pub fn collect() -> Metric {
    let mut system = System::new_all();
    system.refresh_all();

    let cpu_load = system.global_cpu_info().cpu_usage();

    Metric::Gauge {
        name: "cpu_load".to_string(),
        value: cpu_load as f64,
    }
}
