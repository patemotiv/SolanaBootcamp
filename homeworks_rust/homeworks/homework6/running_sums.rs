// Given a vector of integers, compute the running sum where each element
// is the sum of all previous elements including itself.
// See if you can figure out a "Rusty" way to do it using Rust's iterators and methods

fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut sum = 0;
    nums.iter().map(|x| { sum += x; sum }).collect()
}

// Given a integer, compute the fibonacci numbers up to that count
// Do it in a Rusty way using Rust's iterators and methods

fn fibonacci(n: u64) -> Vec<u64> {
    let mut fib = vec![1, 1];
    for i in 2..n {
        fib.push(fib[(i - 1) as usize] + fib[(i - 2) as usize]);
    }
    fib
}

fn main() {  
    println!("{:?}", running_sum(vec![1, 1, 1, 1, 1]));
    
    println!("{:?}", running_sum(vec![1, 2, 3, 4]));
    
    println!("{:?}", running_sum(vec![3, 1, 2, 10, 1]));

    println!("{:?}", fibonacci(60));
}
