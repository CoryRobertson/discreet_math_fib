use crate::NumberSearchResult::NumberNotFound;
use std::fmt::{Display, Formatter};

// limit for fibonacci number calculation
static LIMIT: u128 = 1_000_000;

// type def for contents of the fibonacci list
type FibContents = u128;
// type def for list of fibonacci numbers
type FibList = Vec<FibContents>;

// enum for finding a number in the fibonacci list
#[derive(Debug)]
enum NumberSearchResult {
    _NumberFound(FibContents),
    NumberNotFound,
}

// display implementation for enum
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
    let fib_vec = fib(1000, LIMIT); // create fibonacci number vector with n = 1000 OR up until the LIMIT
    println!("{:?}", fib_vec); // print the list
    print_if_in_fib_series(55, &fib_vec); // find a specific number in the list if it exists

    let range_bottom = 0; // range for starting sum checking
    let range_range = 20; // how many numbers on top of the bottom to do a summation of

    // loop for showing the series of each natural number
    for n in range_bottom..range_bottom + range_range {
        let sum_series = find_sum_of_fib(n, &fib_vec); // generate the sum series
        let sum: FibContents = sum_series.iter().sum(); // generate the sum to check if the sum is equal to the series and the number we checked

        if n != sum {
            panic!("sum not equal to fib number to check");
        } // this should never run under any circumstances, but useful just incase :)

        println!("Series: {:?}", sum_series); // print the series

        println!("Sum of series: {}", sum); // print the sum of the series
    }
}

/// Finds the series of sums of fib numbers to create the given number
fn find_sum_of_fib(number: FibContents, fib_vec: &FibList) -> Vec<FibContents> {
    if fib_vec.contains(&number) || number == 0 || number >= LIMIT {
        return vec![number];
    }

    let mut copy_list = fib_vec.clone(); // a list we can modify throughout the function.
    let mut sum = number; // the sum we are going to work with.
    let mut list_of_sums: Vec<FibContents> = vec![];

    loop {
        for index in 0..copy_list.len() {
            let current_number = copy_list.get(index).unwrap();
            let previous_number: FibContents = {
                if index == 0 {
                    *current_number
                } else {
                    *copy_list.get(index - 1).unwrap()
                }
            };
            if current_number >= &sum {
                // always go to the previous number once we go one number higher than the sum.
                // if we have found the number in the fibonacci sequence that is larger than the number we are trying to make
                list_of_sums.push(previous_number);
                sum -= previous_number;
                if index > 0 {
                    // remove the index we added to the list_of_sums
                    copy_list.remove(index - 1);
                }
                break;
            }
        }

        if sum == 0 {
            break;
        } // once the sum is 0, we have all the numbers to make up the number in the list_of_sums vector

        if copy_list.is_empty() {
            break;
        } // this should never run, unless the number somehow cant be made from the list.
    } // once this loop concludes, we have found our list_of_sums

    list_of_sums
}

/// Print a number if it exists in the fibonacci list, if not, print another message
fn print_if_in_fib_series(number: FibContents, fib_vec: &FibList) {
    match find_fib_series(number, fib_vec) {
        Ok(num) => {
            println!(
                "Number was found at index: {}, number searched for: {}",
                num, number
            );
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
            return Ok((index + 1) as FibContents); // number was found
        }
    }
    Err(NumberNotFound) // number was not found
}

/// Create a Fibonacci list of numbers with up to 1000 numbers, comparing each number to limit and returning  if the number would exceed limit
fn fib(n: FibContents, limit: FibContents) -> FibList {
    let mut vec = vec![1, 1]; // initialize FibList vector with 1,1
    for i in 2..n {
        let new_number = vec.get(i as usize - 1).unwrap() + vec.get(i as usize - 2).unwrap(); // calculate new number to add to fibonacci list, cant fail because vec length is always >= 2
        if new_number >= limit {
            break;
        } else {
            vec.push(new_number);
        }
    }
    vec
}
