
// The function should have a loop counting up to 301
// If the count is divisible by 3, print "fizz"
// If the count is divisible by 5 print "buzz"
// If the count is divisible by 3 and 5 print "fizz buzz"
// At the end print the number of times "fizz buzz" occurred.
pub fn fizz_buzz() {
    let n = 301;
    let mut fizz_buzz_count = 0;
    for i in 0..n {
        match i {
            i if i % 3 == 0 && i % 5 == 0 => {
                println!("fizz buzz");
                fizz_buzz_count += 1;
            }
            i if i % 3 == 0 => println!("fizz"),
            i if i % 5 == 0 => println!("buzz"),
            _ => println!("{}", i),
        }
    }
    println!("Total fizz buz: {:?}", fizz_buzz_count);
}

// We have Vector of integers called nums and a target integer.
// Return the two indices that add up to the target value.
// Thereʼs always one unique solution for each list.
// You canʼt use the same number twice.
// Use a HashMap to make an efficient solution.
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        //println!("i: {}, num: {}", i, num);
        let complement = target - num;
        if let Some(&j) = map.get(&complement) {
            return vec![j as i32, i as i32];
        }
        map.insert(num, i);
    }
    vec![]
}

fn main() {
    println!("Welcome to Bootcamp!");

    fizz_buzz();

    println!("Two Sum: {:?}", two_sum(vec![2, 7, 11, 15,], 9));

    println!("Two Sum: {:?}", two_sum(vec![3, 2, 4,], 6));

    println!("Two Sum: {:?}", two_sum(vec![3, 3,], 6));

    println!("Two Sum: {:?}", two_sum(vec![2, 3, 4, 5,], 9));
}
