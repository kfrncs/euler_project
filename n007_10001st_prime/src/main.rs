fn main() {
    let mut current: usize = 3;
    let mut nth_prime = 1;

    while nth_prime <= 10001 {
        if is_prime(current) && !(nth_prime == 10001) {
            nth_prime += 1;
            println!("{} is #{} prime number", current, nth_prime);
            if nth_prime == 10001 {break;}
        }
        current += 1;
    }
}

fn is_prime(num: usize) -> bool {
    for i in 2..num {
        if num % i == 0 {
            return false;
        }
    }
    true
}
