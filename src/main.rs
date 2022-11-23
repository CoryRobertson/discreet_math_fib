use crate::NumberSearchResult::NumberNotFound;
use std::fmt::{Display, Formatter};

static LIMIT: u128 = 1_000_000;

type FibContents = u128;
type FibList = Vec<FibContents>;

#[derive(Debug)]
enum NumberSearchResult {
    _NumberFound(FibContents),
    NumberNotFound,
}

impl Display for NumberSearchResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            NumberNotFound => {
                write!(f, "Number was not found")
            }
            _ => {
                write!(f, "Number was found")
            }
        }
    }
}

fn main() {

    let fib_vec = fib(1000, LIMIT);
    for n in &fib_vec {
        print!("{}, ", n);
    }
    println!();
    print_if_in_fib_series(55, &fib_vec);

}

fn find_sum_of_fib(number: FibContents, fib_vec: &FibList) -> String {

    if fib_vec.contains(&number) {
        return format!("Series: {}", number);
    }
    let mut copy_list = fib_vec.clone(); // a list we can modify throughout the function.
    let mut sum = number; // the sum we are going to work with.
    let mut list_of_sums: Vec<FibContents> = vec![];

    loop {

        for (index, num) in copy_list.iter().enumerate() {
            if num > &sum { // if we have found the number in the fibonacci sequence that is larger than the number we are trying to make
                let current_num = copy_list.get(index - 1).unwrap(); // the number previous to the one we are looping through
                list_of_sums.push(*current_num);
                sum -= current_num;

            }
        }

    }





}

fn print_if_in_fib_series(number: FibContents, fib_vec: &FibList) {
    match find_fib_series(number, fib_vec) {
        Ok(num) => {
            println!("Number was found at index: {}, number searched for: {}", num, number);
        }
        Err(err) => {
            println!("{}, number searched for: {}", err, number);
        }
    }
}

/// Finds a number in the list and returns its index if it exists, error result if no number is present.
fn find_fib_series(
    number: FibContents,
    fib_list: &FibList,
) -> Result<FibContents, NumberSearchResult> {
    for (index, n) in fib_list.iter().enumerate() {
        if *n == number {
            // index + 1 because prof wants us to start from 1, not 0
            return Ok((index + 1) as FibContents);
        }
    }

    Err(NumberNotFound)
}

/// Create a Fibonacci list of numbers with up to 1000 numbers, comparing each number to limit and returning  if the number would exceed limit
fn fib(n: FibContents, limit: FibContents) -> FibList {
    let mut vec = vec![1, 1];
    for i in 2..n {
        let new_number = vec.get(i as usize - 1).unwrap() + vec.get(i as usize - 2).unwrap();
        if new_number >= limit {
            break;
        } else {
            vec.push(new_number);
        }
    }
    vec
}
