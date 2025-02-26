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


// ------- PROBLEM #1 ------------------------------------------------------------


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


// ------- PROBLEM #2 ------------------------------------------------------------


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
 * in:  a &str, usize, and char
 * out: a String with a char swapped with a specified char at a specified index
 */
pub fn replace_char(input: &str, index: usize, replacement: char) -> String
{
    // set up variables
    let result: String = String::from(input);
    let length: usize = result.len();

    // define the lhs and rhs of the final output
    let mut lhs: String = (&result[0..index]).to_string();
    let rhs: String = (&result[index+1..length]).to_string();
    
    // push specified character onto lhs
    lhs.push(replacement);

    // combine lhs and rhs and return the result
    lhs.push_str(&rhs);
    lhs
}


/**
 * in:  a &str and a usize
 * out: a String with a random printable character inserted at the specified index
 */
pub fn insert_char(input: &str, index: usize, insertion: char) -> String
{
    // set up variables
    let result: String = String::from(input);
    let length: usize = result.len();

    // define lhs and rhs of final output
    let mut lhs: String = (&result[0..index+1]).to_string();
    let rhs: String = (&result[index+1..length]).to_string();

    // push specified character onto lhs
    lhs.push(insertion);

    // combine lhs and rhs and return result
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


/**
 * in:  a &str
 * out: a vector containing every index where a letter appears
 */
pub fn find_letter_indices(input: &str) -> Vec<usize>
{
    // setup output vector and index tracker variables
    let mut indices: Vec<usize> = Vec::new();
    let mut current_index: usize = 0;

    // create an iterable from the characters of the input string
    let converted_input: String = String::from(input);
    let chars: std::str::Chars<'_> = converted_input.chars();

    // loop through each char in the iterable, adding the index of any char found to be a letter
    for char in chars
    {
        if char.is_alphabetic() { indices.push(current_index); }
        current_index += 1;
    }

    indices
}


/**
 * in:  a String
 * out: a random character index of the input String
 */
pub fn get_random_index(s: String) -> usize
{
    let mut r: ThreadRng = rand::thread_rng();

    let len: usize = s.len() - 1;

    let result: usize = r.gen_range(0..=len);
    result
}


/**
 * in:  a vector
 * out: a random element from the input vector
 */
pub fn get_random_value(v: Vec<usize>) -> usize
{
    let mut r: ThreadRng = rand::thread_rng();

    let len: usize = (v.len() - 1);

    let index: usize = r.gen_range(0..=len);
    v[index]
}


/**
 * in:  a &str
 * out: a removes 5 random letters from the string and prints the result to the console
 */
pub fn delete_a_letter(input: &str) -> ()
{
    fn helper(accumulator: String, remaining_recurses: i32) -> String
    {
        let letter_indices: Vec<usize> = find_letter_indices(&accumulator);

        // base cases
        if accumulator.is_empty() { return String::from("<empty string>"); }
        else if letter_indices.len() == 0 || remaining_recurses == 0 { return accumulator; }

        // recursive case
        else
        {
            let new_accumulator: String = delete_char(&accumulator, get_random_value(letter_indices));
            helper(new_accumulator, (remaining_recurses - 1))   
        }
    }

    println!("MODIFIED STRING: {}", helper(String::from(input), 5))
}



pub fn replace_a_letter(input: &str) -> ()
{
    fn helper(accumulator: String, remaining_recurses: i32) -> String
    {
        let letter_indices: Vec<usize> = find_letter_indices(&accumulator);

        // base cases
        if accumulator.is_empty() { return String::from("<empty string> (why in the world would you pass an empty string to this??)"); }
        else if letter_indices.len() == 0 || remaining_recurses == 0 { return accumulator; }

        // recursive case
        else
        {
            let new_accumulator: String = replace_char(&accumulator, get_random_value(letter_indices), get_rand_printable());
            helper(new_accumulator, (remaining_recurses - 1))   
        }
    }

    println!("MODIFIED STRING: {}", helper(String::from(input), 5))
}



pub fn insert_a_letter(input: &str) -> ()
{
    fn helper(accumulator: String, remaining_recurses: i32) -> String
    {
        // base
        if remaining_recurses == 0 { return accumulator; }

        // recursive case
        else
        {
            let new_accumulator: String = insert_char(&accumulator, get_random_index(accumulator.clone()), get_rand_letter());
            helper(new_accumulator, (remaining_recurses - 1))
        }
    }

    println!("MODIFIED STRING: {}", helper(String::from(input), 5))
}

// ------- PROBLEM #3 ------------------------------------------------------------
