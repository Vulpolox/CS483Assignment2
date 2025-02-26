mod utils;

fn main() 
{
    utils::helpers::delete_a_letter("Hello World");
    utils::helpers::insert_a_letter("Hello World");
    utils::helpers::replace_a_letter("Hello World");

    println!("{}", utils::helpers::is_valid_password("Foxgfdsgfsgfdsg!8"));
    println!("{}", utils::helpers::is_valid_username(""));

}
