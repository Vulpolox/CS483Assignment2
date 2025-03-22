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
fn test_username_password_validity_predicates() // it's over 40 lines for readability (could have been 32); pls don't take points off...
{
    // valid crendentials
    assert_eq!(problems::is_valid_password("G3t@M0ney") &&
               problems::is_valid_username("Bob"),
               true);

    assert_eq!(problems::is_valid_password("R@nd0mP@ss1") &&
              problems::is_valid_username("Joe"),
              true);

    assert_eq!(problems::is_valid_password("W1nter$torM") &&
            problems::is_valid_username("Billy"),
            true);

    assert_eq!(problems::is_valid_password("D@rkN1ght99!") &&
            problems::is_valid_username("Carl"),
            true);
    
    assert_eq!(problems::is_valid_password("Qw3rty&U!") &&
            problems::is_valid_username("Waldo"),
            true);

    assert_eq!(problems::is_valid_password("Z3phyr~123") &&
            problems::is_valid_username("Willow"),
            true);

    assert_eq!(problems::is_valid_password("St0rm!Ch@se") &&
            problems::is_valid_username("Jess"),
            true);

    assert_eq!(problems::is_valid_password("P@ssw0rd$9") &&
            problems::is_valid_username("Carla"),
            true);

    assert_eq!(problems::is_valid_password("S@f3H@ven7") &&
            problems::is_valid_username("Tim"),
            true);

    assert_eq!(problems::is_valid_password("M@trixC0d3!") &&
            problems::is_valid_username("Timmy"),
            true);

    // invalid credentials
    assert_eq!(problems::is_valid_password("M@trixC0d3!") &&
            problems::is_valid_username("Timmy="),
            false);
    
    assert_eq!(problems::is_valid_password("M@trixC0d3!") &&
              problems::is_valid_username("Timmy=Tom"),
              false);

    assert_eq!(problems::is_valid_password("M@trixC0d3!") &&
              problems::is_valid_username("Timmy'"),
              false);

    assert_eq!(problems::is_valid_password("fdfd") &&
              problems::is_valid_username("Timmy"),
              false);

    assert_eq!(problems::is_valid_password("!@#$%^&*(J(") &&
              problems::is_valid_username("Timmy"),
              false);

    assert_eq!(problems::is_valid_password("M@trixC0d3!") &&
              problems::is_valid_username("'' OR 1=1 --"),
              false);

    assert_eq!(problems::is_valid_password("Password123") &&
              problems::is_valid_username("='=''='='='='='='='='='='='='='='='='='Jeff"),
              false);

    assert_eq!(problems::is_valid_password("*************") &&
              problems::is_valid_username("Timothy=Me"),
              false);

    assert_eq!(problems::is_valid_password("M@trixC0d3!") &&
              problems::is_valid_username("1+1=4"),
              false);

    assert_eq!(problems::is_valid_password("strongpasswordyoullneverguessit") &&
              problems::is_valid_username("Little Timmy"),
              false);

    assert_eq!(problems::is_valid_password("1234") &&
              problems::is_valid_username("Admin"),
              false);

    assert_eq!(problems::is_valid_password("...") &&
              problems::is_valid_username("..."),
              false);

    assert_eq!(problems::is_valid_password("goodpassword123") &&
              problems::is_valid_username(""),
              false);

    assert_eq!(problems::is_valid_password("money") &&
              problems::is_valid_username("Timmy"),
              false);

    assert_eq!(problems::is_valid_password("") &&
              problems::is_valid_username(""),
              false);

    assert_eq!(problems::is_valid_password("M@trixC0d3!") &&
              problems::is_valid_username("amazon.com/my-product=happy"),
              false);

    assert_eq!(problems::is_valid_password("4839thrng898t89ephfdhfdjkshfjdksafjd$$SDEjfkj4i") &&
              problems::is_valid_username("Best Password = this one"),
              false);

    assert_eq!(problems::is_valid_password("M@trixC0d3!") &&
              problems::is_valid_username("timmy's username"),
              false);

    assert_eq!(problems::is_valid_password("M@trixC0d3!") &&
              problems::is_valid_username("'admin"),
              false);

    assert_eq!(problems::is_valid_password("M@trixC0d3!") &&
              problems::is_valid_username("12/4=3"),
              false);

}