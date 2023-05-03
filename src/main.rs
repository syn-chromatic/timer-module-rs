mod test;
mod timer_module;

use test::algorithm_test::{binary_search_value, generate_array};
use timer_module::profiler::TimeProfiler;
use timer_module::timer::TimerModule;

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut profiler: TimeProfiler = TimeProfiler::new(false);

    profile_function(&mut profiler);
    profiler.print_profiling_report();

    timer_example();
}

fn profile_function(profiler: &mut TimeProfiler) {
    let array: Vec<i32> = generate_array(10_000);
    let mut function3 = profiler.function_wrapper(|(a, b)| binary_search_value(a, b));

    for value in array.iter() {
        function3((&array, *value));
    }
}

fn timer_example() {
    let mut timer_module = TimerModule::new();
    timer_module.start();
    sleep(Duration::from_millis(100));
    println!("Timer Example: {:?}", timer_module)
}
