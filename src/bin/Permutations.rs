// https://cses.fi/problemset/task/1070

fn main() {
    let mut inp = String::new();
    std::io::stdin()
        .read_line(&mut inp)
        .expect("Error in taking input");
    let n: u64 = inp.trim().parse().expect("Input not a number");
    match n {
        0 => println!("NO SOLUTION "),
        1 => println!("1"),
        2..=3 => println!("NO SOLUTION "),
        _ => {
            for i in (2..=n).step_by(2) {
                print!("{} ", i);
            }
            for i in (1..=n).step_by(2) {
                print!("{} ", i);
            }
        }
    }
}
