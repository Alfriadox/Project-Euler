use super::q_3; // use is_prime from q_3
pub fn main() {
    println!("{}", sum_primes(2_000_000));
}

fn old_sum_primes(n: u64) -> u64 {
    let is_prime = q_3::is_prime; // function used from question 3
    let mut sum: u64 = 2;
    let mut x = 1;
    while x < n-2 {
        if is_prime(x) {
            sum += x;
        }
        x += 2;
        println!("{}", x);
    }
    return sum;
}

fn sum_primes(n: usize) -> u64 {
    // create vector the size of the number of primes that need to be sorted through
    let mut prime_vec: Vec<bool> = vec![true; n];
    prime_vec[1] = false;
    // for every number below it that is prime, make all multiples not prime
    for ind in 2..prime_vec.len() {
        if prime_vec[ind] {
            let mut jump_number: u64 = 2;
            while (ind as u64)*jump_number < n as u64 {
                prime_vec[ind*jump_number as usize] = false;
                jump_number += 1;
            }
        }
    }
    let mut sum: u64 = 0;
    for ind in 2..prime_vec.len() {
        if prime_vec[ind] {
            sum += ind as u64;
        }
    }
    return sum;
}