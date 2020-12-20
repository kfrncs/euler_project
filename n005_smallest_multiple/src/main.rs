fn main() {
    let mut i: usize = 20;
    // This number is 1 less than usize's limit
    while i < std::usize::MAX { 
        if is_divisible_by_all(i) {
            println!("{} is divisible by all numbers between 1 and 20.", i);
            break
        }
        i += 20;
    }
}
        
fn is_divisible_by_all(n: usize) -> bool {
    for j in (2..20) {
        if n % j != 0 {
            println!("{} is not divisible by {}", n, j);
            return false;
        }
    }
    true
}
