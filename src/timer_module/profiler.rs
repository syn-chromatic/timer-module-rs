use std::any::TypeId;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant};

use super::metrics::CallableMetrics;
use super::metrics::ProfileMetricsReport;

pub struct TimeProfiler {
    realtime: bool,
    callable_refs: Arc<Mutex<HashMap<u64, CallableMetrics>>>,
    timing_refs: Arc<Mutex<HashMap<u64, HashMap<u64, CallableMetrics>>>>,
    pcall_hash: Option<u64>,
}

impl TimeProfiler {
    pub fn new(realtime: bool) -> Self {
        TimeProfiler {
            realtime,
            callable_refs: Arc::new(Mutex::new(HashMap::new())),
            timing_refs: Arc::new(Mutex::new(HashMap::new())),
            pcall_hash: None,
        }
    }

    pub fn print_report(&self) {
        let mut metrics_report: ProfileMetricsReport = ProfileMetricsReport::new(self.realtime);
        let callable_refs: MutexGuard<HashMap<u64, CallableMetrics>> =
            self.callable_refs.lock().unwrap();
        let timing_refs: MutexGuard<HashMap<u64, HashMap<u64, CallableMetrics>>> =
            self.timing_refs.lock().unwrap();

        metrics_report.write_report(&callable_refs, &timing_refs);
        drop(callable_refs);
        drop(timing_refs);
    }

    fn append_metrics(&mut self, call_hash: u64, time: Duration) {
        let mut callable_refs: MutexGuard<HashMap<u64, CallableMetrics>> =
            self.callable_refs.lock().unwrap();
        let mut timing_refs: MutexGuard<HashMap<u64, HashMap<u64, CallableMetrics>>> =
            self.timing_refs.lock().unwrap();

        let time_ns = time.as_nanos() as f64;
        let pcall_hash = self.pcall_hash;

        callable_refs.get_mut(&call_hash).unwrap().time_ns += time_ns;
        callable_refs.get_mut(&call_hash).unwrap().ncalls += 1;

        if let Some(pcall_hash) = pcall_hash {
            if call_hash == pcall_hash {
                let call_metrics = callable_refs.get_mut(&call_hash).unwrap();
                call_metrics.time_ns += time_ns;
                call_metrics.ncalls += 1;
                self.pcall_hash = None;

                if self.realtime {
                    self.print_report();
                }
            } else {
                let call_metrics = timing_refs
                    .get_mut(&pcall_hash)
                    .unwrap()
                    .get_mut(&call_hash)
                    .unwrap();

                call_metrics.time_ns += time_ns;
                call_metrics.ncalls += 1;
            }
        }
        drop(callable_refs);
        drop(timing_refs);
    }

    fn create_callable_metrics<F>(&self, call: &F, call_hash: u64) -> CallableMetrics {
        let name: &String = &format!("{:p}", &call);
        let module: &str = "wrapped_function";

        let call_metrics = CallableMetrics {
            name: String::from(name),
            module: String::from(module),
            call_hash,
            ncalls: 0,
            time_ns: 0.0,
        };
        call_metrics
    }

    fn hash_type_id(&self, type_id: TypeId) -> u64 {
        let mut hasher: DefaultHasher = DefaultHasher::new();
        type_id.hash(&mut hasher);
        hasher.finish()
    }

    fn set_pcall_hash(&mut self, call_hash: u64) {
        let callable_refs: MutexGuard<HashMap<u64, CallableMetrics>> =
            self.callable_refs.lock().unwrap();
        let mut timing_refs: MutexGuard<HashMap<u64, HashMap<u64, CallableMetrics>>> =
            self.timing_refs.lock().unwrap();

        let mut pcall_hash = self.pcall_hash;

        if pcall_hash.is_none() {
            pcall_hash = Some(call_hash);
            self.pcall_hash = pcall_hash;
            if !timing_refs.contains_key(&call_hash) {
                timing_refs.insert(call_hash, HashMap::new());
            }
            return;
        }

        let pcall_hash_value = pcall_hash.unwrap();
        let pcall_timing = timing_refs.get_mut(&pcall_hash_value).unwrap();
        if !pcall_timing.contains_key(&call_hash) {
            let call_metrics = callable_refs.get(&call_hash).unwrap().clone();
            let new_metrics = call_metrics.clone_and_reset();
            pcall_timing.insert(call_hash, new_metrics);
        }
    }

    fn add_call_ref<F, A, R>(&mut self, call: &F) -> u64
    where
        F: Fn(A) -> R + Send + Sync + 'static,
        A: Send + Sync,
        R: 'static,
    {
        let type_id: TypeId = TypeId::of::<F>();
        let call_hash: u64 = self.hash_type_id(type_id);
        let call_metrics: CallableMetrics = self.create_callable_metrics(call, call_hash);
        let mut callable_refs: MutexGuard<HashMap<u64, CallableMetrics>> =
            self.callable_refs.lock().unwrap();
        callable_refs.insert(call_hash, call_metrics);
        call_hash
    }

    pub fn function_wrapper<F, A, R>(&mut self, function: F) -> impl FnMut(A) -> R + '_
    where
        F: Fn(A) -> R + Send + Sync + 'static,
        A: Send + Sync,
        R: 'static,
    {
        let call_hash: u64 = self.add_call_ref(&function);

        move |arg: A| {
            self.set_pcall_hash(call_hash);
            let start_time: Instant = Instant::now();
            let result: R = function(arg);
            let elapsed_time: Duration = start_time.elapsed();
            self.append_metrics(call_hash, elapsed_time);
            result
        }
    }
}
