use crate::NumberSearchResult::{NumberExceededLimit, NumberFound, NumberNotFound};
use num_bigint::BigUint;
use std::fmt::{Display, Formatter};
use std::io::{stdin};
use std::mem::replace;
use std::time::SystemTime;

// limit for fibonacci number calculation
static LIMIT: u128 = 1_000_000;

// type def for contents of the fibonacci list
type FibContents = u128;
// type def for list of fibonacci numbers
type FibList = Vec<FibContents>;

// enum for finding a number in the fibonacci list
#[derive(Debug)]
enum NumberSearchResult {
    NumberFound(FibContents),
    NumberNotFound,
    NumberExceededLimit,
}

// display implementation for enum
impl Display for NumberSearchResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            NumberNotFound => {
                write!(f, "Number was not found")
            }
            NumberExceededLimit => {
                write!(f, "Number exceeded limit")
            }
            _ => { write!(f, "Number was found") }
        }
    }
}

fn main() {
    let start = SystemTime::now(); // take a timestamp at the start of the program
    let fib_vec = fib(1000, LIMIT); // create fibonacci number vector with n = 1000 OR up until the LIMIT
    println!("{:?}", fib_vec); // print the list

    find_sum_of_fib(999_999,&fib_vec).unwrap();

    { // a scope us used here so we never accidentally reuse the search_fib variable
        let msg = "Input a valid number to search for in the fibonacci sequence: ";
        let search_fib: FibContents = badger_user_for_number(msg);
        print_if_in_fib_series(search_fib, &fib_vec); // find a specific number in the list if it exists
    } // block for searching for a specific number in the fibonacci sequence

    {
        let msg = "Input a number to make sum of fibonacci numbers: ";
        let break_fib = badger_user_for_number(msg);
        match find_sum_of_fib(break_fib,&fib_vec) {
            Ok(sum_series) => {
                let sum_of_series: FibContents = sum_series.iter().sum();
                println!("Sum of series: {}", sum_of_series);
                println!("Series: {:?}", sum_series);
            }
            Err(err) => {
                println!("{}", err);
            }
        }

    }

    println!();

    // #[cfg(debug_assertions)]
    // {
    //     let range_bottom = 0; // range for starting sum checking
    //     let range_range = 1; // how many numbers on top of the bottom to do a summation of
    //
    //     // println!("Big fib number: {}", _fib_specific(1_000_000));
    //
    //     // loop for showing the series of each natural number
    //     for n in range_bottom..=range_bottom + range_range {
    //         let sum_series = find_sum_of_fib(n, &fib_vec); // generate the sum series
    //         let sum: FibContents = sum_series.iter().sum(); // generate the sum to check if the sum is equal to the series and the number we checked
    //
    //         if n != sum {
    //             panic!("sum not equal to fib number to check");
    //         } // this should never run under any circumstances, but useful just incase :)
    //         println!("Sum of series: {}", sum); // print the sum of the series
    //         println!("Series ^: {:?}\n", sum_series); // print the series
    //     }
    // }

    let end = SystemTime::now(); // take a timestamp at the end of the program.

    let diff = end.duration_since(start).unwrap(); // calculate difference of time between beginning of the program and end
    println!("Program took {} seconds to run.", diff.as_secs_f64());
}

fn badger_user_for_number(message: &str) -> FibContents {
    return loop { // number to search for is looped upon until we have a working number of type FibContents
        let mut input = String::new();
        println!("{}", message);
        let string_result = stdin().read_line(&mut input); // after requesting user input, read into a buffer with a mutable pointer
        let trim_input = input.trim(); // trim input e.g. user inputs "10", string read would be "10\0" with a null terminator at the end, trimming turns it into "55" so we can parse it.
        let number_result = trim_input.parse::<FibContents>(); // result for parsing the number
        match string_result {
            Ok(_len) => { // string was able to be read, this check contains length checking
                match number_result {
                    Ok(num) => { // number was able to parse into FibContents.
                        break num; // break loop and return anonymously the number that we parsed
                    }
                    Err(err) => {
                        println!("Invalid input parse error. {}", err); // number was not able to be parsed, print the error to the user and loop again.
                    }
                }
            }
            Err(err) => {
                println!("Invalid input read line error. {}", err); // input was not able to be read, print the error to the user and loop again.
            }
        }
    };
}

/// Finds the series of sums of fib numbers to create the given number
fn find_sum_of_fib(number: FibContents, fib_vec: &FibList) -> Result<Vec<FibContents>,NumberSearchResult> {
    if fib_vec.contains(&number) || number == 0 {
        return Ok(vec![number]);
    }
    if number >= LIMIT {
        return Err(NumberExceededLimit);
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
                break; // this break makes us stop and go back to the beginning of the vector once we have subtracted once, absolutely needed.
            }
        }

        // println!("copy list len: {}", copy_list.len());
        // println!("copy list: {:?}", copy_list);
        // println!("sum: {}", sum);
        // println!("list of sums: {:?}", list_of_sums);

        if sum == 0 {
            break;
        } // once the sum is 0, we have all the numbers to make up the number in the list_of_sums vector

        if copy_list.is_empty() {
            break;
        } // this should never run, unless the number somehow cant be made from the list.
    } // once this loop concludes, we have found our list_of_sums

    Ok(list_of_sums)
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

/// Finds specific fibonacci number given an index, uses BigUint so it can be a massive number
fn _fib_specific(n: usize) -> BigUint {
    let mut a0: BigUint = BigUint::from(0_u32);
    let mut a1: BigUint = BigUint::from(1_u32);

    for _ in 0..n {
        let a2 = a0 + &a1;

        a0 = replace(&mut a1, a2);
    }

    a0
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
