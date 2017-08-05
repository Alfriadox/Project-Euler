pub fn main() {
    // old code
    /*
    let mut gpf = 0;
    let v = get_factors(600851475143);
    //println!("{:?}", v);
    let mut ind = 0;
    let l = v.len();
    for x in v {
        println!("Checking factor {} out of {}.", ind, l);
        ind+=1;
        if is_prime(x) && x > gpf {
            gpf = x;
        }
    }
    */
    println!("\n{}", get_greatest_prime_factor(600851475143));
}

fn get_factors(n: u64) -> Vec<u64> {
    let mut v = vec![];
    for x in 2..n/2+1 {
        println!("Checking if {} is a factor.", x);
        if n%x == 0 {
            v.push(x);
        }
    }
    v
}

fn is_prime(n: u64) -> bool { // at worst O(n)
    for x in 2..n/2+1 {
        if n%x == 0 {
            return false;
        }
    }
    return true;
}

fn get_greatest_prime_factor(n: u64) -> u64 { // O((n/4)*(n+1))
    let mut f : u64 = 0;
    let mut current: u64 = 3;
    while current <= n/2+1 {
        if is_prime(current) && n%current == 0 {
            f = current;
            println!("{}", f);
            // cancel after it stops
        }
        current+=2;
        //println!("Checking {}.", current);
    }
    return f;
}