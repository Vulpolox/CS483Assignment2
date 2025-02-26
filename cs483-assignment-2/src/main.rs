mod utils;

fn main() 
{
    let c: char = utils::helpers::get_rand_printable();
    println!("Random character: {:?}", c);

}
