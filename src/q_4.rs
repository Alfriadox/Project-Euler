pub fn main() {
    let mut p = 101;
    for x in 100..1000 {
        for y in 100..1000 {
            if is_palindrome(x*y) && x*y > p {
                p = x*y;
                println!("{}",p);
            }
        }
    }
}

fn is_palindrome(num: u64) -> bool {
    let num_string = num.to_string();
    let half = num_string.len() / 2;
    num_string.bytes().take(half).eq(num_string.bytes().rev().take(half))
}