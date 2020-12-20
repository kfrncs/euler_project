fn main() {
    let mut largest = 0;
    
    for i in (0..999).rev() {
        for j in (0..999).rev() {
            let mut prod = i * j;
            let mut rev = prod.to_string().chars().rev().collect::<String>();
            if rev == prod.to_string() && prod > largest {
                largest = prod;
                println!("Setting largest to {}", largest);
            }
        }
    }
}
