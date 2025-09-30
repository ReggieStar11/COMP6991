use std::env;
use std::num::ParseIntError;

struct TribonacciError(String);

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let error_message = String::from("Please enter a valid size");

    let size = match args.get(1) {
        Some(s) => s.parse::<usize>(),
        None => Ok(10),
    };

    if let Err(e) = compute_tribonacci(size, error_message) {
        println!("Error: {}", e.0)
    }

    
}

/// Computes the tribonacci sequence of a given size
/// Prints the sequence, and its sum
fn compute_tribonacci(
    size: Result<usize, ParseIntError>,
    // The error message your function should return
    // inside the `TribonacciError` struct
    error_msg: String,
) -> Result<(), TribonacciError> {
    
    
    let target_size = match size {
        Ok(n) => n,
        Err(_) => return Err(TribonacciError(error_msg)),  
    };
   
    // let target_size= size.unwrap();

    if target_size > 145 {
        return Err(TribonacciError(error_msg));
    }

        
    let mut numbers: Vec<i128> = vec![1,1,1];
    let mut num_size = numbers.len();

    while num_size < target_size {
        let first = numbers[num_size-1];
        let second = numbers[num_size-2];
        let third = numbers[num_size-3];
        
        let sum = first + second + third;
        
        // println!("{}", sum);
        numbers.push(sum);
        num_size = numbers.len();
    }
    
    
    // println!("{:?}", numbers);
    // TODO: complete this function!
    println!("Values: {:?}", numbers);
    Ok(())

}
