fn main() {
    let (mut a, mut b, mut c, mut sum) = (1, 2, 0, 0);
    
    while a < 4_000_000 {
        if a % 2 == 0 {
            sum += a;
        }
        c = a + b;
        a = b;
        b = c;
    }
    println!("{}", sum);
}
