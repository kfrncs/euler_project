fn main() {
    let target: usize = 600851475143;
    println!("{}", largest_prime_factor(target));
}

fn largest_prime_factor(n: usize) -> usize {
    let mut largest: usize = 0;
    let mut factors = Vec::new();

    for i in 1.. ((n as f64).sqrt() as usize) {
        if n % i == 0 {
            factors.push(i);
            println!("Adding {} to factors.", i);
        }
    }

    for x in factors {
        if is_prime(x) && x > largest {
            largest = x;
        }
    }

    largest
}


fn is_prime(y: usize) -> bool {
    for z in 2..y {
        if y % z == 0 {return false;}
    }
    return true
}
