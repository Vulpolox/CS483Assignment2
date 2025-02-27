use crate::utils::problems;

#[test]
fn test_is_palindrome()
{
    // valid palindromes
    assert_eq!(problems::is_palindrome("Racecar"), true);
    assert_eq!(problems::is_palindrome("22/02/2022"), true);
    assert_eq!(problems::is_palindrome("Was it a car or a cat I saw?"), true);
    assert_eq!(problems::is_palindrome("A man, a plan, a canal, Panama!"), true);
    assert_eq!(problems::is_palindrome("A dog! A panic in a pagoda!"), true);
    assert_eq!(problems::is_palindrome("A Toyota's a Toyota"), true);
    assert_eq!(problems::is_palindrome("Never odd or even"), true);
    assert_eq!(problems::is_palindrome("Kayak"), true);
    assert_eq!(problems::is_palindrome("Civic"), true);
    assert_eq!(problems::is_palindrome("Level"), true);

    // invalid palindromes
    assert_eq!(problems::is_palindrome("12345"), false);
    assert_eq!(problems::is_palindrome("palindrome"), false);
    assert_eq!(problems::is_palindrome("UMKC"), false);
    assert_eq!(problems::is_palindrome("..."), false);
    assert_eq!(problems::is_palindrome("flaky"), false);
    assert_eq!(problems::is_palindrome("rust"), false);
    assert_eq!(problems::is_palindrome("dkslf;lafkd"), false);
    assert_eq!(problems::is_palindrome("boss"), false);
    assert_eq!(problems::is_palindrome("joe"), false);
    assert_eq!(problems::is_palindrome("willy"), false);
    assert_eq!(problems::is_palindrome("lewis"), false);
    assert_eq!(problems::is_palindrome("judy"), false);
    assert_eq!(problems::is_palindrome("testing"), false);
    assert_eq!(problems::is_palindrome("30requiredtests"), false);
    assert_eq!(problems::is_palindrome("bored"), false);
    assert_eq!(problems::is_palindrome("fingers"), false);
    assert_eq!(problems::is_palindrome("soda"), false);
    assert_eq!(problems::is_palindrome("ripe"), false);
    assert_eq!(problems::is_palindrome("apple"), false);
    assert_eq!(problems::is_palindrome("fjdkfjdkslfjkdslfjkdlsa;jfkldsjfkl;dsjfkld;sjfkldsjfkldjskfldjskl;fjdsklfhioehfeo"), false);
}





#[test]
fn test_delete_a_letter()
{
    // required test strings for problem #2
    let s1 = "GET /index.html HTTP/1.1";
    let s2 = "https://www.umkc.edu/current-students/";
    let s3 = "https://en.wikipedia.org/wiki/America_Fuzzy_Lop_(software)";
    let s4 = "http://www.google.com";

    println!("\nNow Testing delete_a_letter()\n---");
    println!("ORIGINAL STRING: {}", s1);
    problems::delete_a_letter(s1);

    println!("\nORIGINAL STRING: {}", s2);
    problems::delete_a_letter(s2);

    println!("\nORIGINAL STRING: {}", s3);
    problems::delete_a_letter(s3);

    println!("\nORIGINAL STRING: {}", s4);
    problems::delete_a_letter(s4);
}


#[test]
fn test_replace_a_letter()
{
    // required test strings for problem #2
    let s1 = "GET /index.html HTTP/1.1";
    let s2 = "https://www.umkc.edu/current-students/";
    let s3 = "https://en.wikipedia.org/wiki/America_Fuzzy_Lop_(software)";
    let s4 = "http://www.google.com";

    println!("\nNow Testing replace_a_letter()\n---");
    println!("ORIGINAL STRING: {}", s1);
    problems::replace_a_letter(s1);

    println!("\nORIGINAL STRING: {}", s2);
    problems::replace_a_letter(s2);

    println!("\nORIGINAL STRING: {}", s3);
    problems::replace_a_letter(s3);

    println!("\nORIGINAL STRING: {}", s4);
    problems::replace_a_letter(s4);
}


#[test]
fn test_insert_a_letter()
{
    // required test strings for problem #2
    let s1 = "GET /index.html HTTP/1.1";
    let s2 = "https://www.umkc.edu/current-students/";
    let s3 = "https://en.wikipedia.org/wiki/America_Fuzzy_Lop_(software)";
    let s4 = "http://www.google.com";

    println!("\nNow Testing insert_a_letter()\n---");
    println!("ORIGINAL STRING: {}", s1);
    problems::insert_a_letter(s1);

    println!("\nORIGINAL STRING: {}", s2);
    problems::insert_a_letter(s2);

    println!("\nORIGINAL STRING: {}", s3);
    problems::insert_a_letter(s3);

    println!("\nORIGINAL STRING: {}", s4);
    problems::insert_a_letter(s4);
}


#[test]
fn test_username_password_validity_predicates()
{
    
}