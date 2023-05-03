mod test;
mod timer_module;

use crate::test::algorithm_test::{binary_search_value, generate_array};
use crate::timer_module::profiler::TimeProfiler;

fn main() {
    let mut profiler = TimeProfiler::new(false);

    let array: Vec<i32> = generate_array(100_000);
    {
        let mut function3 = profiler.function_wrapper(|(a1, a2)| binary_search_value(a1, a2));
        for value in array.iter() {
            function3((&array, *value));
        }
    }

    profiler.print_profiling_report();
}
