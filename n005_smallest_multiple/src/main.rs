fn main() {
    let mut i: usize = 1;
    // This number is 1 less than usize's limit
    for i in 1..std::usize::MAX { 
        if is_divisible_by_all(i) {
            println!("{} is divisible by all numbers between 1 and 20.", i);
            break
        }
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
