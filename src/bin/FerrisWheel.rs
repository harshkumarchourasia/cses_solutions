// https://cses.fi/problemset/task/1090

fn main() {
    let mut input = String::new();

    // Read the first line of input
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Parse the first line into integers
    let sizes: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Not an integer"))
        .collect();

    let mut n = sizes[0]; // The first integer
    let x = sizes[1]; // The second integer

    input.clear();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mut p: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Not an integer"))
        .collect();
    p.sort();
    let mut start: usize = 0;
    let mut end: usize = n - 1;
    while start < end {
        if p[start] + p[end] <= x {
            n -= 1;
            start += 1;
            end -= 1;
        } else {
            end -= 1;
        }
    }
    print!("{}", n);
}
