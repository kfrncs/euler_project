fn main() {
    let (mut sum_of_squares, mut square_of_sum) = (0, 0);
    for i in 1..101 {
        sum_of_squares += i32::pow(i, 2);
        println!("adding {}. sum_of_squares is {}",  i32::pow(i, 2), sum_of_squares);
    }
    
    for x in 1..101 {
        square_of_sum += x;
        println!("adding {}. square_of_sum is {}", x, square_of_sum)
    }
    square_of_sum = i32::pow(square_of_sum, 2);


    println!("Sum of squares: {}", sum_of_squares);
    println!("Square of sum: {}", square_of_sum);
    println!("{}", (square_of_sum - sum_of_squares));
}
