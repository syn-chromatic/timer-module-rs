mod test;
mod timer_module;

use test::algorithm_test::{binary_search_value, generate_array};
use timer_module::profiler::TimeProfiler;

fn main() {
    let mut profiler: TimeProfiler = TimeProfiler::new(true);

    profile_function(&mut profiler);
    profiler.print_profiling_report();
}

fn profile_function(profiler: &mut TimeProfiler) {
    let array: Vec<i32> = generate_array(10_000);
    println!("STARTING");

    let mut function3 = profiler.function_wrapper(|(_, _)| binary_search_value);

    for value in array.iter() {
        function3((&array, *value));
    }
}
