// https://cses.fi/problemset/task/1068/
use std;

fn main() {
    let mut inp=String::new();
    std::io::stdin().read_line(&mut inp).expect("Failed to read line");
    let mut n:u64 = inp.trim().parse().expect("Input not an integer");
    while n > 1 {

        print!("{} ", n);
        if n % 2 == 0 {
            n = n / 2
        } else {
            n = 3 * n + 1
        }
    }
    print!("{}", 1);
}