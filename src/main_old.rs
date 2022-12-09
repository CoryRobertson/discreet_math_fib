use crate::main_old::NumberSearchResult::{
    NumberExceededLimit, NumberExceededSumOfFib, NumberNotFound, _NumberFound,
};
use num_bigint::BigUint;
use std::fmt::{Display, Formatter};
use std::io::stdin;
use std::mem::replace;

// limit for fibonacci number calculation imposed by assignment
pub static LIMIT: u128 = 1_000_000;

// type def for contents of the fibonacci list
pub type FibContents = u128;
// type def for list of fibonacci numbers
pub type FibList = Vec<FibContents>;

// enum for finding a number in the fibonacci list
#[derive(Debug)]
pub enum NumberSearchResult {
    _NumberFound(FibContents), // number was found, shouldn't be used most likely
    NumberNotFound,            // number was somehow not able to be found
    NumberExceededLimit,       // number was larger than the limit imposed by the assignment
    NumberExceededSumOfFib, // number was larger than the sum of the list, making it impossible to make
}

pub fn string_from_fib_list(list: &FibList, selected_number: FibContents) -> String {
    let mut s = String::new();
    for (index, num) in list.iter().enumerate() {
        // look through each number in the fibonacci list
        if index == list.len() - 1 {
            // add a comma if we are not at the end of the list
            if *num == selected_number {
                // if the number in the list matches the number the user was looking for, highlight it with square brackets.
                s.push_str(&format!("[{}]", num))
            } else {
                s.push_str(&format!("{}", num))
            }
        } else if *num == selected_number {
            // if the number in the list matches the number the user was looking for, highlight it with square brackets.
            s.push_str(&format!("[{}], ", num))
        } else {
            s.push_str(&format!("{}, ", num))
        }
    }
    s
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
            _NumberFound(num) => {
                write!(f, "Number was found: {}", num)
            }

            NumberExceededSumOfFib => {
                write!(f, "Number exceeded sum of fib numbers up to limit")
            }
        }
    }
}

// fn main() {
//     let fib_vec = fib(1000, LIMIT); // create fibonacci number vector with n = 1000 OR up until the LIMIT
//     println!("{:?}", fib_vec); // print the list
//     let fib_sum: FibContents = fib_vec.iter().sum();
//     println!("sum of fib: {}", fib_sum); // print the list
//
//     {
//         // a scope us used here so we never accidentally reuse the search_fib variable
//         let msg = "Input a valid number to search for in the fibonacci sequence: ";
//         let search_fib: FibContents = badger_user_for_number(msg);
//         print_if_in_fib_series(search_fib, &fib_vec); // find a specific number in the list if it exists
//     } // block for searching for a specific number in the fibonacci sequence
//
//     {
//         let msg = "Input a number to make sum of fibonacci numbers: ";
//         let break_fib = badger_user_for_number(msg);
//         match find_sum_of_fib(break_fib, &fib_vec) {
//             Ok(sum_series) => {
//                 let sum_of_series: FibContents = sum_series.iter().sum();
//                 println!("Sum of series: {}", sum_of_series);
//                 println!("Series: {:?}", sum_series);
//             }
//             Err(err) => {
//                 println!("{}", err);
//             }
//         }
//     } // block for  allowing the user to display the sum of fibs for a number
//
//     println!();
//
//     let _start = SystemTime::now(); // take a timestamp at the _start of the program for speed testing
//
//     #[cfg(debug_assertions)]
//     {
//         let end = SystemTime::now(); // take a timestamp at the end of the program.
//
//         let diff = end.duration_since(_start).unwrap(); // calculate difference of time between beginning of the program and end
//
//         println!("Program took {} seconds to run.", diff.as_secs_f64());
//     } // debug block to show print time of all fib numbers,
// }

/// Request user input until they input a valid FibContents.
/// Prints message each time they fail :)
/// Not needed to be tested.
pub fn badger_user_for_number(message: &str) -> FibContents {
    return loop {
        // number to search for is looped upon until we have a working number of type FibContents
        let mut input = String::new();
        println!("{}", message);
        let string_result = stdin().read_line(&mut input); // after requesting user input, read into a buffer with a mutable pointer
        let trim_input = input.trim(); // trim input e.g. user inputs "10", string read would be "10\0" with a null terminator at the end, trimming turns it into "55" so we can parse it.
        let number_result = trim_input.parse::<FibContents>(); // result for parsing the number
        match string_result {
            Ok(_len) => {
                // string was able to be read, this check contains length checking
                match number_result {
                    Ok(num) => {
                        // number was able to parse into FibContents.
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
/// Tested for whole range.
pub fn find_sum_of_fib(
    number: FibContents,
    fib_vec: &FibList,
) -> Result<Vec<FibContents>, NumberSearchResult> {
    {
        // includes:
        // number already is a fib number,
        // number is larger than imposed limit,
        // number is larger than the sum of all the fib numbers, should not happen if limit is lower than it, but I plan on making limit higher
        if fib_vec.contains(&number) || number == 0 {
            return Ok(vec![number]);
        } // number is a fibonacci number already
        if number >= LIMIT {
            let sum_of_fib: FibContents = fib_vec.iter().sum();
            if sum_of_fib < number {
                // nested error return so we can find out why the number failed and display to user. :)
                return Err(NumberExceededSumOfFib);
            } // number is larger than any of the numbers
            return Err(NumberExceededLimit);
        } // number exceeds limit
    } // sanity checks, run once only

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

            if copy_list.contains(&sum) && sum != 0 {
                list_of_sums.push(sum);
                sum = 0;
                break;
            } // check if the number we are looking for is even in the list, if so we can stop looping through entirely.

            if current_number >= &sum || (index == copy_list.len() - 1) {
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

        if sum == 0 {
            return Ok(list_of_sums.clone());
        } // once the sum is 0, we have all the numbers to make up the number in the list_of_sums vector

        if copy_list.is_empty() {
            return Err(NumberNotFound);
        } // this should never run, unless the number somehow cant be made from the list.
    } // once this loop concludes, we have found our list_of_sums
}

/// Print a number if it exists in the fibonacci list, if not, print another message
/// Not needed to be tested
pub fn print_if_in_fib_series(number: FibContents, fib_vec: &FibList) {
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
/// Tested
pub fn find_fib_series(
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
/// Untested as of now, not needed to be tested either, known to work
pub fn _fib_specific(n: usize) -> BigUint {
    let mut a0: BigUint = BigUint::from(0_u32);
    let mut a1: BigUint = BigUint::from(1_u32);

    for _ in 0..n {
        let a2 = a0 + &a1;

        a0 = replace(&mut a1, a2);
    }

    a0
}

/// Create a Fibonacci list of numbers with up to 1000 numbers, comparing each number to limit and returning  if the number would exceed limit
/// Tested
pub fn fib(n: FibContents, limit: FibContents) -> FibList {
    let mut vec = vec![1, 1]; // initialize FibList vector with 1,1
    for i in 2..n {
        let new_number = vec.get(i as usize - 1).unwrap() + vec.get(i as usize - 2).unwrap(); // calculate new number to add to fibonacci list, cant fail because vec length is always >= 2
        if new_number >= limit {
            break;
        } else {
            vec.push(new_number);
        }
    }
    // 1,1,2,3,5
    vec
}

mod tests {
    #![allow(unused_imports)]
    use crate::main_old::{
        _fib_specific, fib, find_fib_series, find_sum_of_fib, FibContents, LIMIT,
    };
    use num_bigint::BigUint;
    use rayon::prelude::*;

    /// This test is fast, checks that find fib series actually locates the correct number in the list of fib numbers.
    #[test]
    fn test_find_fib_specific() {
        let fib_vec = fib(1000, LIMIT);

        for number in &fib_vec {
            let found_number = find_fib_series(*number, &fib_vec).unwrap();
            let indexed_number = fib_vec.get(found_number as usize - 1).unwrap();
            assert_eq!(number, indexed_number); // compare each number into the fib list to a number in a known list location
        }
    }

    /// Tests fib(n,limit), short runtime test that checks each number with a known working function that can calculate fib numbers given an index.
    #[test]
    fn test_specific_fib_for_list() {
        let fib_vec = fib(1000, LIMIT); // create a fib list that may or may not work correctly.
        for (index, number) in fib_vec.iter().enumerate() {
            let big_uint = BigUint::from(*number); // make a big uint out of the number in this list, for comparison reasons
            let specific = _fib_specific(index + 1); // run the known working function to take in an index and return a fib number
            assert_eq!(specific, big_uint); // check numbers for validity
        }
    }

    /// This test takes a long time to run, and can sometimes hang on an ide, run using bash instead.
    #[test]
    fn test_number_range() {
        let fib_vec = fib(1000, LIMIT); // generate the fib list
        let range_bottom = 0; // range for starting sum checking
        let range_top = LIMIT - 1; // how many numbers on top of the bottom to do a summation of

        // using a parallel iterator speeds up this unit test by a substantial amount.
        (range_bottom..range_bottom + range_top)
            .into_par_iter()
            .for_each(|n| {
                let sum_series = find_sum_of_fib(n, &fib_vec).unwrap(); // generate the sum series
                let sum: FibContents = sum_series.iter().sum(); // generate the sum to check if the sum is equal to the series and the number we checked

                for number in &sum_series {
                    let mut count = 0;
                    for second_number in &sum_series {
                        if number == second_number {
                            count += 1;
                        }
                    }
                    assert_eq!(count, 1); // each number in the sum series appears once and only once
                }
                assert_eq!(n, sum); // number being made from sum of series should be equal to the number sent in to the function.
            });
    }
}
