pub fn main() {
    println!("{}", sum_mults(1000));
}

fn sum_mults(n: u32) -> u64 {
    let mut v = vec![];
    for x in 1..n {
        if (x%3 == 0) || (x%5==0) {
            v.push(x);
        }
    }
    let mut sum: u64 = 0;
    for x in v {
        sum += x as u64;
    }
    sum
}