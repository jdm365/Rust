use std::time::{Instant};

fn run_collatz(mut initial: u64) -> (u64, u64) {
    let original = initial;

    let mut steps: u64 = 0;

    while initial != 1 {
        if initial % 2 == 0 {
            initial = &initial / 2;
            steps += 1;
        }
        else {
            initial = (3 * &initial + 1) / 2;
            steps += 2;
        }
    }
    return (original, steps);
}

fn run_collatz_range(min: u64, max: u64) {
    if max <= min {
        panic!("Max number must be greater than min number.");
    }

    let mut largest: u64 = 0;

    for i in min..max {
        let (_original, _steps) = run_collatz(i);
        let original = _original.to_string();
        let steps = _steps.to_string();
        
        if _steps > largest {
            largest = _steps;
            println!("{original} took {steps} steps.");
        }
    }
}



fn main() {
    let start = Instant::now();
    run_collatz_range(1, 10000000);
    let duration = start.elapsed();

    println!("Time taken: {:?}", duration);
}
