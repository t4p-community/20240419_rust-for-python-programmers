fn main() {
    let nums = 0..10;

    // len would be the preferred way to calculate the length of a range
    let len_of_nums = nums.len();
    println!("Length of nums 1: {}", len_of_nums);

    // But what if we wanted to manually calculate the length of a range?
    // We could use the fold method to do so
    // fold moves the range, so we need to clone it
    // move means it takes ownership of the value which mean we can't use it again
    // so we clone it to make a copy and move the clone
    // |acc, _| acc + 1 is the closure that is passed to the fold method
    // Rust closures are similar to lambdas in other languages
    let len_of_nums = nums.clone().fold(0, |acc, _| acc + 1);

    // impl Range {
    //     fn fold(self, init: T, f: F) -> T {
    //         let mut acc = init;
    //         for x in self {
    //             acc = f(acc, x);
    //         }
    //         acc
    //     }
    // }

    println!("Length of nums 2: {}", len_of_nums);

    // sum is a method that is available on ranges to calculate the sum of the range
    let sum_of_nums: i32 = nums.clone().sum();
    println!("Sum of nums 1: {}", sum_of_nums);

    // reduce is similar to fold, but it returns an Option and does not support
    // an initial value
    let sum_of_nums = nums.reduce(|acc, num| acc + num).unwrap();
    println!("Sum of nums 2: {}", sum_of_nums);
}
