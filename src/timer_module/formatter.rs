use std::time::Duration;

pub fn format_time(duration: Duration) -> String {
    let nanos: f64 = duration.as_nanos() as f64;
    if nanos >= 1e9 {
        let secs: f64 = nanos / 1e9;
        format!("{:.2}s", secs)
    } else if nanos >= 1e6 {
        let millis: f64 = nanos / 1e6;
        format!("{:.2}ms", millis)
    } else if nanos >= 1e3 {
        let micros: f64 = nanos / 1e3;
        format!("{:.2}μs", micros)
    } else {
        format!("{:.2}ns", nanos)
    }
}
