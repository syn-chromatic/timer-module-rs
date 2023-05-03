___
## 🖥️ Timer Usage:
```rust
use std::thread::sleep;
use std::time::Duration;
use timer_module::timer::TimerModule;

fn main() {
    let mut timer_module = TimerModule::new();

    timer_module.start();
    sleep(Duration::from_millis(100));

    timer_module.pause();
    sleep(Duration::from_millis(100));

    let time_secs: f64 = timer_module.get_time();
    let time_millis: f64 = timer_module.get_time_ms();
    let formatted: String = timer_module.get_string();

    // Debug trait implements 'get_string' method.
    println!("Time: {:?}", timer_module);
}
```

#### Set the timer
```rust
timer_module.set_time(5).start();
```

#### Refresh time (preserves timer state):
```rust
timer_module.refresh();
```

#### Reset time (resets everyting)
```rust
timer_module.reset();
```

___
## 🖥️ Profiler Usage:
```rust
use std::thread::sleep;
use std::time::Duration;
use timer_module::profiler::TimeProfiler;

fn main() {
    let mut profiler: TimeProfiler = TimeProfiler::new(false);

    profile_function(&mut profiler);
    profiler.print_profiling_report();
}

fn function_sleep(millis: u64) {
    sleep(Duration::from_millis(millis));
}

fn profile_function(profiler: &mut TimeProfiler) {
    let mut wrapped_function = profiler.function_wrapper(|arg| function_sleep(arg));

    for _ in 0..5 {
        wrapped_function(100);
    }
}
```
