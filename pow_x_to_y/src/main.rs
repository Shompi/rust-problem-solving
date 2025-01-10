pub fn my_pow(x: f64, n: i32) -> f64 {

    let mut res = 1.0;
    let mut n = n;
    let mut x = x;
    loop {
        if n % 2 != 0 {
            
            res = if n > 0 {res * x} else {res / x};
            x = x*x;
            n = n / 2;
        }

        if n == 0 {
            break;
        }
    }
    res
}

fn main() {
    println!("Hello, world!");
    println!("{} to the power of {} is: {}", 2, 5, my_pow(2.0, 5));
    println!("{} to the power of {} is: {}", 5, 0, my_pow(5.0, 0));
    println!("{} to the power of {} is: {}", -2, 2, my_pow(-2.0, 2));
    println!("{} to the power of {} is: {}", 9, -2, my_pow(9.0, -2));
}
