// https://cses.fi/problemset/task/1084/

fn main() {
    let mut input = String::new();

    // Read the first line of input
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Parse the first line into integers
    let sizes: Vec<i64> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Not an integer"))
        .collect();

    let n = sizes[0]; // The first integer
    let m = sizes[1]; // The second integer
    let k = sizes[2]; // The third integer

    input.clear();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mut a: Vec<i64> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Not an integer"))
        .collect();
    a.sort();

    input.clear();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mut b: Vec<i64> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Not an integer"))
        .collect();
    b.sort();

    let mut i: i64 = 0;
    let mut j: i64 = 0;
    let mut res = 0;
    while i < n && j < m {
        if (a[i as usize] - b[j as usize]).abs() <= k {
            i += 1;
            j += 1;
            res += 1;
        } else if a[i as usize] - b[j as usize] > k {
            j += 1;
        } else {
            i += 1;
        }
    }
    println!("{}", res);
}
