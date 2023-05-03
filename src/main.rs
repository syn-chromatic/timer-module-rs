mod algorithm_test;
mod profiler;

use crate::algorithm_test::{binary_search_value, generate_array};
use crate::profiler::TimeProfiler;
use lazy_static::lazy_static;

fn main() {
    let mut profiler = TimeProfiler::new(false);
    lazy_static! {
        static ref ARRAY: Vec<i32> = generate_array(100_000);
    }

    let mut function3 = profiler.function_wrapper(|(a1, a2)| binary_search_value(a1, a2));
    for value in ARRAY.iter() {
        function3((&ARRAY, *value));
    }
    drop(function3);

    profiler.print_profiling_report();
}
