// https://cses.fi/problemset/task/1633

fn main() {
    let mut inp = String::new();
    std::io::stdin()
        .read_line(&mut inp)
        .expect("Error in taking input");
    let n: u64 = inp.trim().parse().expect("Input not a number");
    let mut arr: [u64; 10_usize.pow(6) + 1] = [0; 10_usize.pow(6) + 1];
    arr[0] = 1;
    arr[1] = 1;
    if n < 2 {
        println!("{}", arr[n as usize]);
    } else {
        for i in 2..=n {
            for j in 1..=6 {
                if (i as i64 - j as i64) >= 0 {
                    arr[i as usize] += arr[(i - j) as usize];
                    arr[i as usize] %= 10_u64.pow(9) + 7;
                }
            }
        }
        println!("{}", arr[n as usize]);
    }
}
