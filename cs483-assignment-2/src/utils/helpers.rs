use rand::prelude::*;
use core::str;

// SOURCES:

// filter function documentation
// https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter

// rng learning sources
// https://stackoverflow.com/questions/19671845/how-can-i-generate-a-random-number-within-a-range-in-rust
// https://rust-random.github.io/book/guide-start.html
// https://rust-random.github.io/book/quick-start.html
// https://docs.rs/rand/latest/rand/

// int (ASCII) -> char
// https://www.reddit.com/r/rust/comments/2veszg/u8_to_char_using_ascii_encoding/
// https://www.ascii-code.com/


/**
 * in:  a &str
 * out: a bool indicating whether the input is a palindrome
 */
pub fn is_palindrome(input: &str) -> bool
{
    // convert the input to lowercase
    let lowercase_input: String = input.to_lowercase();
    // create a new string that only contains alphanumeric characters
    let filtered_input: String = lowercase_input.chars()
                                                .filter(|x: &char| (x.is_alphabetic() || x.is_numeric())).collect::<String>();
    // return boolean got from comparison of the two strings
    filtered_input == filtered_input.chars().rev().collect::<String>()
}


/** 
 * in:  a &str and a usize
 * out: a String with the character at the specified index removed 
 */
pub fn delete_char(input: &str, index: usize) -> String
{
    // set up variables
    let result: String = String::from(input);
    let length: usize = result.len();

    // define the left and right hand sides of the final output
    let mut lhs: String = (&result[0..index]).to_string();
    let rhs: String = (&result[index+1..length]).to_string();
    
    // combine the lhs and rhs and return the result
    lhs.push_str(&rhs);
    lhs
}


/**
 * in:  a &str and a usize
 * out: a String with a random printable character swapped in for the character at the specified index
 */
pub fn replace_char(input: &mut str, index: usize) -> String
{
    // set up variables
    let result: String = String::from(input);
    let length: usize = result.len();

    // define the lhs and rhs of the final output
    let mut lhs: String = (&result[0..index]).to_string();
    let rhs: String = (&result[index+1..length]).to_string();
    
    // add a random printable character to the lhs
    lhs.push(get_rand_printable());

    // combine lhs and rhs and return the result
    lhs.push_str(&rhs);
    lhs
}


/**
 * in:  a &str and a usize
 * out: a String with a random printable character inserted at the specified index
 */
pub fn insert_char(input: &mut str, index: usize) -> String
{
    // set up variables
    let result: String = String::from(input);
    let length: usize = result.len();

    // define lhs and rhs
    let mut lhs: String = (&result[0..index+1]).to_string();
    let rhs: String = (&result[index+1..length]).to_string();

    // push a random letter onto the lhs, combine lhs
    // and rhs and return result
    lhs.push(get_rand_letter());
    lhs.push_str(&rhs);
    lhs
}


/**
 * in:  no args
 * out: a random printable character (ASCII [32, 127])
 */
pub fn get_rand_printable() -> char
{
    let mut r: ThreadRng = rand::thread_rng();
    
    let i: u8 = r.gen_range(32..=127);
    i as char
}


/**
 * in:  no args
 * out: a random letter (ASCII [65, 90] and [97, 122])
 */
pub fn get_rand_letter() -> char
{
    let mut r: ThreadRng = rand::thread_rng();

    // generate a random capital and lowercase letter
    // and a random integer in range [0, 1]
    let i_cap: u8 = r.gen_range(65..=90);
    let i_lower: u8 = r.gen_range(97..=122);
    let selector: i32 = r.gen_range(0..=1);

    // return one of these two characters at random
    // based on the random integer
    if selector == 0 { i_cap as char }
    else { i_lower as char }
}

