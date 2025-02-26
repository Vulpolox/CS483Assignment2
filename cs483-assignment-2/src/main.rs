mod utils;

fn main() 
{
    let s : &str = "racecarl";
    println!("{} is {} with the tailing l removed", utils::helpers::delete_a_letter(s, 7), s);

}
