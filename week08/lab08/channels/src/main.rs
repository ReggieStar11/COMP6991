use itertools::Itertools;
use std::sync::mpsc;

fn main() {
    let input_number = std::env::args().nth(1).expect("provide five-digit number");
    let _n = input_number.parse::<u32>().expect("invalid number");

    let digits_vec: Vec<i32> = input_number
        .chars()
        .map(|x| x.to_digit(10).expect("not a digit") as i32)
        .collect();

    if digits_vec.len() != 5 {
        panic!("expected a five-digit input");
    }

    let operators = vec!['+', '-', '*', '/'];

    // generate all permutations of the 5 digits and all permutations of 4 operators
    let digits_perms: Vec<Vec<i32>> = digits_vec
        .iter()
        .permutations(5)
        .map(|p| p.into_iter().copied().collect::<Vec<_>>())
        .collect();

    let ops_perms: Vec<Vec<char>> = operators
        .iter()
        .permutations(4)
        .map(|p| p.into_iter().copied().collect::<Vec<_>>())
        .collect();

    // cartesian product of digit permutations and operator permutations
    let digits_operators: Vec<(Vec<i32>, Vec<char>)> = digits_perms
        .into_iter()
        .cartesian_product(ops_perms.into_iter())
        .collect();

    println!(
        "There are {} potential combinations",
        digits_operators.len()
    );

    let num_chunks = 6usize;
    let chunk_size = (digits_operators.len() + num_chunks - 1) / num_chunks;
    let chunks: Vec<Vec<(Vec<i32>, Vec<char>)>> = digits_operators
        .chunks(chunk_size)
        .map(|c| c.to_vec())
        .collect();

    let (tx, rx) = mpsc::channel::<(usize, usize)>();

    std::thread::scope(|scope| {
        for (i, chunk) in chunks.into_iter().enumerate() {
            let tx = tx.clone();
            scope.spawn(move || {
                let mut local_count: usize = 0;
                for (digits, operators) in chunk {
                    if calculate(digits, operators) {
                        local_count += 1;
                    }
                }
                let _ = tx.send((i, local_count));
            });
        }
    });

    drop(tx);

    let mut total = 0usize;
    for (id, count) in rx {
        println!("Thread {} found {} combinations", id, count);
        total += count;
    }
    println!("Total: {}", total);
}

fn calculate(digits: Vec<i32>, operators: Vec<char>) -> bool {
    if digits.len() != 5 || operators.len() != 4 {
        return false;
    }

    let num1 = digits[0];
    let num2 = digits[1];
    let num3 = digits[2];
    let num4 = digits[3];
    let num5 = digits[4];

    let op1 = operators[0];
    let op2 = operators[1];
    let op3 = operators[2];
    let op4 = operators[3];

    if let Ok(mut result) = operate(num1, num2, op1) {
        if let Ok(r2) = operate(result, num3, op2) {
            result = r2;
            if let Ok(r3) = operate(result, num4, op3) {
                result = r3;
                if let Ok(r4) = operate(result, num5, op4) {
                    result = r4;
                    if result == 10 {
                        println!(
                            "{} {} {} {} {} {} {} {} {} = 10",
                            num1, op1, num2, op2, num3, op3, num4, op4, num5
                        );
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn operate(num1: i32, num2: i32, op: char) -> Result<i32, ()> {
    match op {
        '+' => Ok(num1 + num2),
        '-' => Ok(num1 - num2),
        '*' => Ok(num1 * num2),
        '/' => num1.checked_div(num2).ok_or(()),
        _ => Err(()),
    }
}
