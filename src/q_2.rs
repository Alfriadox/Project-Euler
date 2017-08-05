pub fn main() {
    let mut fibb: Vec<u64> = vec![1,2];
    while fibb.last().unwrap() < &4_000_000 {
        let fib = fibb.clone();
        fibb.push(fib[fib.len()-1]+fib[fib.len()-2]);
    }
    fibb.pop();
    let mut sum: u64 = 0;
    for f in fibb {
        if f%2 == 0 {
            sum+=f;
        }
    }
    println!("{}", sum);
}

