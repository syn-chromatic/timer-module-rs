use super::terminal::ANSICode;
use super::terminal::Terminal;

use core::hash::Hasher;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::MutexGuard;
use std::time::Duration;

pub struct TimeFormatterNs {
    nanos: f64,
}

impl TimeFormatterNs {
    pub fn new(nanos: f64) -> TimeFormatterNs {
        TimeFormatterNs { nanos }
    }

    pub fn new_from_duration(duration: Duration) -> TimeFormatterNs {
        let nanos: f64 = duration.as_nanos() as f64;
        TimeFormatterNs { nanos }
    }

    pub fn format_seconds(&self) -> String {
        let secs: f64 = self.nanos / 1e9;
        format!("{:.2}s", secs)
    }

    pub fn format_milliseconds(&self) -> String {
        let millis: f64 = self.nanos / 1e6;
        format!("{:.2}ms", millis)
    }

    pub fn format_microseconds(&self) -> String {
        let micros: f64 = self.nanos / 1e3;
        format!("{:.2}μs", micros)
    }

    pub fn format_nanoseconds(&self) -> String {
        format!("{:.2}ns", self.nanos)
    }

    pub fn auto_format(&self) -> String {
        let nanos: f64 = self.nanos;
        if nanos >= 1e9 {
            return self.format_seconds();
        } else if nanos >= 1e6 {
            return self.format_milliseconds();
        } else if nanos >= 1e3 {
            return self.format_microseconds();
        }
        self.format_nanoseconds()
    }
}

#[derive(Debug)]
pub struct CallableMetrics {
    pub name: String,
    pub module: String,
    pub call_hash: u64,
    pub ncalls: usize,
    pub time_ns: f64,
}

impl CallableMetrics {
    pub fn new(
        name: String,
        module: String,
        call_hash: u64,
        ncalls: usize,
        time_ns: f64,
    ) -> CallableMetrics {
        CallableMetrics {
            name,
            module,
            call_hash,
            ncalls,
            time_ns,
        }
    }

    pub fn clone_and_reset(&self) -> Self {
        let call_metrics: CallableMetrics = CallableMetrics::new(
            self.name.clone(),
            self.module.clone(),
            self.call_hash,
            0,
            0.0,
        );
        call_metrics
    }

    pub fn get_percall_time(&self) -> f64 {
        if self.ncalls > 0 {
            let percall_time_ns = self.time_ns / self.ncalls as f64;
            return percall_time_ns;
        }
        0.0
    }
}

impl PartialEq for CallableMetrics {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.module == other.module
    }
}

impl Eq for CallableMetrics {}

impl Hash for CallableMetrics {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.module.hash(state);
    }
}

pub struct ProfileMetricsReport {
    terminal: Terminal,
    header_color: ANSICode,
    call_color: ANSICode,
    total_time_color: ANSICode,
}

impl ProfileMetricsReport {
    pub fn new(realtime: bool) -> ProfileMetricsReport {
        let terminal = Terminal::new();
        let header_color = Self::get_header_color(realtime);
        let call_color = Self::get_call_color(realtime);
        let total_time_color = Self::get_total_time_color(realtime);

        ProfileMetricsReport {
            terminal,
            header_color,
            call_color,
            total_time_color,
        }
    }

    fn get_header_color(realtime: bool) -> ANSICode {
        if realtime {
            return ANSICode::Yellow;
        }
        ANSICode::Green
    }

    fn get_call_color(realtime: bool) -> ANSICode {
        if realtime {
            return ANSICode::Cyan;
        }
        ANSICode::White
    }

    fn get_total_time_color(realtime: bool) -> ANSICode {
        if realtime {
            return ANSICode::Yellow;
        }
        ANSICode::Green
    }

    fn get_relative_percentage(&self, pcall_ns: f64, call_ns: f64) -> f64 {
        let mut percentage = 0.0;
        if pcall_ns > 0.0 && call_ns > 0.0 {
            percentage = (call_ns / pcall_ns) * 100.0;
        }
        percentage
    }

    fn write_primary_call_header(&mut self, call_metrics: &CallableMetrics) {
        let pcall_name = &call_metrics.name;
        let profile_header = format!("█ PROFILE: {} █", pcall_name);
        let separator = "=".repeat(profile_header.len());
        let string = format!("\n{}\n{}", profile_header, separator);
        self.terminal.set_ansi_color(self.header_color);
        self.terminal.write(&string);
    }

    fn write_primacy_call_report(&mut self, pcall_metrics: &CallableMetrics) {
        let pcall_time_ns = pcall_metrics.time_ns;
        let pcall_ncalls = pcall_metrics.ncalls;
        let percall_time_ns = pcall_metrics.get_percall_time();

        let pcall_time = TimeFormatterNs::new(pcall_time_ns).auto_format();
        let percall_time = TimeFormatterNs::new(percall_time_ns).auto_format();

        let string = format!(
            "Profile Time: [{}]\nNCalls: [{}] — PerCall: [{}]\n——————\n",
            pcall_time, pcall_ncalls, percall_time
        );
        self.terminal.set_ansi_color(self.call_color);
        self.terminal.write(&string);
    }

    fn write_call_report(&mut self, call_metrics: &CallableMetrics, pcall_time: f64) {
        let call_name = &call_metrics.name;
        let call_time_ns = call_metrics.time_ns;
        let call_ncalls = call_metrics.ncalls;
        let percall_time_ns = call_metrics.get_percall_time();

        let prc = self.get_relative_percentage(pcall_time, call_time_ns);
        let call_time = TimeFormatterNs::new(call_time_ns).auto_format();
        let percall_time = TimeFormatterNs::new(percall_time_ns).auto_format();

        let string = format!(
            "Name: {}\nTime: [{}] — T%: {:.2}%\nNCalls: [{}] — PerCall: [{}]\n——",
            call_name, call_time, prc, call_ncalls, percall_time
        );
        self.terminal.set_ansi_color(self.call_color);
        self.terminal.write(&string);
    }

    fn get_total_time(
        &self,
        callable_refs: &MutexGuard<HashMap<u64, CallableMetrics>>,
        timing_refs: &MutexGuard<HashMap<u64, HashMap<u64, CallableMetrics>>>,
    ) -> f64 {
        let mut total_time = 0.0;
        for (pcall_hash, _) in timing_refs.iter() {
            total_time += callable_refs.get(pcall_hash).unwrap().time_ns;
        }
        total_time
    }

    pub fn write_report(
        &mut self,
        callable_refs: &MutexGuard<HashMap<u64, CallableMetrics>>,
        timing_refs: &MutexGuard<HashMap<u64, HashMap<u64, CallableMetrics>>>,
    ) {
        for (pcall_hash, subcalls) in timing_refs.iter() {
            let pcall_metrics = callable_refs.get(pcall_hash).unwrap();
            self.write_primary_call_header(pcall_metrics);
            let pcall_time = pcall_metrics.time_ns;

            for (_, subcall_metrics) in subcalls.iter() {
                if subcall_metrics == pcall_metrics {
                    continue;
                }
                self.write_call_report(subcall_metrics, pcall_time);
            }
            self.write_primacy_call_report(pcall_metrics);
        }
        let total_time_ns = self.get_total_time(callable_refs, timing_refs);
        let total_time = TimeFormatterNs::new(total_time_ns).auto_format();

        let string = format!("――― Total Time: [{}] ―――\n\n\n", total_time);
        self.terminal.set_ansi_color(self.total_time_color);
        self.terminal.write(&string);
    }
}
