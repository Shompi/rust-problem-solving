use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

fn fibonacci(n: i32) -> i64 {
    // Lo principal es que fibonacci se calcula con el caso base
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    // Luego, para calcular las otras secuencias, calculamos el fib de
    // fib(n - 1) + fib(n - 2)

    let mut sum = 0;

    sum += fibonacci(n - 1) + fibonacci(n - 2);

    sum
}

fn fibonacci_with_memo(n: i32) -> i64 {
    let mut memo: Vec<i64> = vec![];

    if n == 0 {
        return 0;
    };
    if n == 1 {
        return 1;
    };

    memo.push(0);
    memo.push(1);

    for i in 2..n + 1 {
        // Compute the value
        let result = memo[(i - 1) as usize] + memo[(i - 2) as usize];

        // add this result to the memo vector at positions n-1 and n-2
        memo.push(result);
    }

    *memo.last().unwrap()
}

fn fibonacci_with_memo_by_ref(n: i32, memo: &HashMap<i32, i64>) -> i64 {
    if n == 0 {
        return 0;
    };

    if n == 1 {
        return 1;
    };

    let result;

    if memo.contains_key(&n) {
        return *memo.get(&n).unwrap();
    }

    if n >= 2 {
        result =
            fibonacci_with_memo_by_ref(n - 1, &memo) + fibonacci_with_memo_by_ref(n - 2, &memo);
    } else {
        result = 1;
    }
    result
}

fn main() {

    let mut start_time:Instant;
    let mut elapsed: u128;
    let target = 40;
    let mut result = 0;

    println!("Calculating fib of {} recursively", target);
    start_time = Instant::now();
    result = fibonacci(target);
    elapsed = Duration::from(start_time.elapsed()).as_millis();
    println!("Result = {} Time Elapsed: {} ms", result, elapsed);

    println!("Calculating fib with memo (not global) of {}", target);
    start_time = Instant::now();
    result = fibonacci_with_memo(target);
    elapsed = Duration::from(start_time.elapsed()).as_millis();
    println!("Result = {} Time Elapsed: {} ms", result, elapsed);


    println!("Calculating fib of {} with memo by ref", target);
    start_time = Instant::now();
    let mut memo: HashMap<i32, i64> = HashMap::new();
    for i in 0..target+1 {
        let result = fibonacci_with_memo_by_ref(i, &memo);
        memo.insert(i, result);
    }
    result = *memo.get(&target).unwrap();
    elapsed = Duration::from(start_time.elapsed()).as_millis();
    println!("Result = {} Time Elapsed: {} ms", result, elapsed);
}
