use super::formatter::format_time;

use std::any::TypeId;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct ObjectCall {
    name: String,
    module: String,
    time: Duration,
    ncalls: u32,
}

impl PartialEq for ObjectCall {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.module == other.module
    }
}

impl Eq for ObjectCall {}

impl Hash for ObjectCall {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.module.hash(state);
    }
}

pub struct TimeProfiler {
    realtime: bool,
    prof_timing_refs: HashMap<u64, Vec<u64>>,
    prof_timing_total: Duration,
    object_refs: Arc<Mutex<HashMap<u64, ObjectCall>>>,
    pcall_obj: Option<u64>,
}

impl TimeProfiler {
    pub fn new(realtime: bool) -> Self {
        TimeProfiler {
            realtime,
            prof_timing_refs: HashMap::new(),
            prof_timing_total: Duration::new(0, 0),
            object_refs: Arc::new(Mutex::new(HashMap::new())),
            pcall_obj: None,
        }
    }

    fn append_object_profiling(&mut self, object_hash: u64, time: Duration) {
        let mut object_refs: MutexGuard<HashMap<u64, ObjectCall>> =
            self.object_refs.lock().unwrap();
        object_refs.get_mut(&object_hash).unwrap().time += time;
        object_refs.get_mut(&object_hash).unwrap().ncalls += 1;

        drop(object_refs);

        if self.pcall_obj.is_some() {
            if object_hash == self.pcall_obj.unwrap() {
                self.prof_timing_total += time;
                self.pcall_obj = None;

                if self.realtime {
                    self.print_profiling_report();
                }
            }
        }
    }

    fn create_object_call<F>(&self, object: &F) -> ObjectCall {
        let name: &String = &format!("{:p}", &object);
        let module: &str = "wrapped_function";

        let object_call = ObjectCall {
            name: String::from(name),
            module: String::from(module),
            time: Duration::new(0, 0),
            ncalls: 0,
        };
        object_call
    }

    fn hash_type_id(&self, type_id: TypeId) -> u64 {
        let mut hasher: DefaultHasher = DefaultHasher::new();
        type_id.hash(&mut hasher);
        hasher.finish()
    }

    fn print_pcall_header(&self, object_call: &ObjectCall) {
        let pcall_name: &String = &object_call.name;
        let profile_header: String = format!("█ PROFILE: {} █", pcall_name);
        let header_len: usize = profile_header.len();
        let header: String = "=".repeat(header_len);
        println!("\n{}\n{}", profile_header, header)
    }

    fn print_pcall(&self, object_call: &ObjectCall) {
        let pcall_time: Duration = object_call.time;
        let pcall_ncalls: u32 = object_call.ncalls;
        let pcall_percall: Duration = pcall_time / pcall_ncalls;

        let f_pcall_time: String = format_time(pcall_time);
        let f_pcall_percall: String = format_time(pcall_percall);

        println!(
            "Profile Time: [{}]\nNCalls: [{}] — PerCall: [{}]\n——————\n",
            f_pcall_time, pcall_ncalls, f_pcall_percall
        );
    }

    fn print_call(&self, object_call: &ObjectCall, pcall_time: Duration) {
        let obj_name: &String = &object_call.name;
        let obj_time: Duration = object_call.time;

        let obj_ncalls: u32 = object_call.ncalls;
        let obj_percall: Duration = obj_time / obj_ncalls;

        let f_obj_time: String = format_time(obj_time);
        let f_obj_percall: String = format_time(obj_percall);

        let mut t_prc: f64 = 0.0;

        if !obj_time.is_zero() && !pcall_time.is_zero() {
            let obj_time_ns: f64 = obj_time.as_nanos() as f64;
            let pcall_time_ns: f64 = pcall_time.as_nanos() as f64;
            t_prc = (obj_time_ns / pcall_time_ns) * 100.0;
        }

        println!(
            "Name: {}\nTime: [{}] — T%: {:.2}%\nNCalls: [{}] — PerCall: [{}]\n——",
            obj_name, f_obj_time, t_prc, obj_ncalls, f_obj_percall
        )
    }

    pub fn print_profiling_report(&self) {
        let object_refs: MutexGuard<HashMap<u64, ObjectCall>> = self.object_refs.lock().unwrap();

        for (pcall_hash, obj_vec) in self.prof_timing_refs.iter() {
            let pcall_object: &ObjectCall = object_refs.get(pcall_hash).unwrap();
            self.print_pcall_header(pcall_object);
            let pcall_time: Duration = pcall_object.time;
            for object_hash in obj_vec {
                if object_hash == pcall_hash {
                    continue;
                }
                let object_call: &ObjectCall = object_refs.get(object_hash).unwrap();
                self.print_call(object_call, pcall_time);
            }
            self.print_pcall(pcall_object);
        }

        let time_total: String = format_time(self.prof_timing_total);
        println!("――― Total Time: [{}] ―――\n\n\n", time_total);
    }

    fn set_pcall_object(&mut self, object_hash: u64) -> u64 {
        if self.pcall_obj.is_none() {
            self.pcall_obj = Some(object_hash);
            self.prof_timing_refs.insert(object_hash, Vec::new());
        }

        let pcall_hash: u64 = self.pcall_obj.unwrap();
        self.prof_timing_refs
            .get_mut(&pcall_hash)
            .unwrap()
            .push(object_hash);

        object_hash
    }

    fn add_object_ref<F, A, R>(&mut self, object: &F) -> u64
    where
        F: Fn(A) -> R + Send + Sync + 'static,
        A: Send + Sync,
        R: 'static,
    {
        let type_id: TypeId = TypeId::of::<F>();
        let hash_ref: u64 = self.hash_type_id(type_id);
        let object_call: ObjectCall = self.create_object_call(&object);

        if !self.object_refs.lock().unwrap().contains_key(&hash_ref) {
            self.object_refs
                .lock()
                .unwrap()
                .insert(hash_ref, object_call);
        }
        hash_ref
    }

    pub fn function_wrapper<F, A, R>(&mut self, function: F) -> impl FnMut(A) -> R + '_
    where
        F: Fn(A) -> R + Send + Sync + 'static,
        A: Send + Sync,
        R: 'static,
    {
        let hash_ref: u64 = self.add_object_ref(&function);

        move |arg: A| {
            let hash_ref: u64 = self.set_pcall_object(hash_ref);
            let start_time: Instant = Instant::now();
            let result: R = function(arg);
            let elapsed_time: Duration = start_time.elapsed();
            self.append_object_profiling(hash_ref, elapsed_time);
            result
        }
    }
}
