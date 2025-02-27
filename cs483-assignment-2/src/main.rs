mod utils;

fn main() 
{
    utils::problems::delete_a_letter("Hello World");
    utils::problems::insert_a_letter("Hello World");
    utils::problems::replace_a_letter("Hello World");

    println!("{}", utils::problems::is_valid_password("Foxgfdsgfsgfdsg!8"));
    println!("{}", utils::problems::is_valid_username(""));

}
