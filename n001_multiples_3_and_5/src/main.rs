fn main() {
    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            println!("{}", i);
        }
    }
}
