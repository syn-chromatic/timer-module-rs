mod test;
mod timer_module;

use test::algorithm_test::{binary_digits, generate_binary_combinations};
use test::algorithm_test::{binary_search_value, generate_array};

use timer_module::profiler::TimeProfiler;
use timer_module::timer::TimerModule;

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut profiler: TimeProfiler = TimeProfiler::new(false);

    profile_function(&mut profiler);
    profile_function2(&mut profiler);
    profile_function3(&mut profiler);

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

fn profile_function2(profiler: &mut TimeProfiler) {
    let mut function3 = profiler.function_wrapper(binary_digits);

    function3(12);
}

fn profile_function3(profiler: &mut TimeProfiler) {
    let mut function3 = profiler.function_wrapper(generate_binary_combinations);

    function3(12);
}

fn timer_example() {
    let mut timer_module = TimerModule::new();
    timer_module.start();
    sleep(Duration::from_millis(100));
    println!("Timer Example: {:?}", timer_module)
}
