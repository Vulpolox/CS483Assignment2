use std::rand;
use std::rand::rng;

// filter function documentation
// https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter

// rng learning source
// https://stackoverflow.com/questions/19671845/how-can-i-generate-a-random-number-within-a-range-in-rust

use core::str;

// in:  a &str
// out: a bool indicating whether the input is a palindrome
pub fn is_palindrome(input: &str) -> bool
{
    // convert the input to lowercase
    let lowercase_input: String = input.to_lowercase();
    // create a new string that only contains alphanumeric characters
    let filtered_input: String = lowercase_input.chars().filter(|x: &char| 
                                                                (x.is_alphabetic() || x.is_numeric())) // use lambda to filter out non-alphanumeric characters
                                                                .collect::<String>(); // convert the iterable back to a string
    filtered_input == filtered_input.chars().rev().collect::<String>() // compare the filtered input to its reverse
}

// in:  a &str and an i32
// out: a String with the character at the specified index removed
pub fn delete_a_letter(input: &str, index: usize) -> String
{
    let result: String = String::from(input);
    let length: usize = result.len();
    let mut lhs: String = (&result[0..index]).to_string();
    let rhs: String = (&result[index+1..length]).to_string();
    lhs.push_str(&rhs);
    lhs
}

// in:  a &str and an i32
// out: a String with a random printable character swapped in for the 
//      character at the specified index
pub fn replace_a_letter(input: &mut str, index: usize) -> String
{
    let result: String = String::from(input);
    let length: usize = result.len();
    let mut lhs: String = (&result[0..index]).to_string();
    let rhs: String = (&result[index+1..length]).to_string();
    lhs.push(get_rand_char());
    lhs.push_str(&rhs);
    lhs
}

// in:  a &str and an i32
// out: a String with a random printable character inserted at the
//      specified index
pub fn insert_a_letter(input: &mut str, index: usize) -> String
{
    let result: String = String::from(input);
    let length: usize = result.len();
    let mut lhs: String = (&result[0..index+1]).to_string();
    let rhs: String = (&result[index+1..length]).to_string();
    lhs.push(get_rand_char());
    lhs.push_str(&rhs);
    lhs
}


pub fn get_rand_char() -> char
{
    let mut rng = rand::rng();
    rng.random::<char>()
}