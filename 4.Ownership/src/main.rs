fn main() {
    let x = 5;
    let y = x;         //copy

    let s1 = String::from("hello");
    let s2 = s1;   //move
    println!("{}, world!", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone();   //copy
    println!("{}, world!", s1);

    //ownership
    //Giving ownership
    let s = String::from("hello");
    takes_ownership(s);
    //println!("{}", s);     ->   This will give error as ownership of s is transferred to takes_ownership

    //copying
    let x = 5;
    makes_copy(x);
    println!("{}", x);       //  This will not give error as primities are copied

    //Getting ownership
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s1 = {}, s3 = {}", s1, s3);

    //Reference 
    //Length calculation without reference
    let s1 = String::from("hello");
    let (s2, len) = calculate_length1(s1);
    println!("The length of {} is {}", s2, len);

    //Length calculation with reference (Borrowing)
    let s1 = String::from("hello");
    let len = calculate_length2(&s1);
    println!("The length of {} is {}", s2, len);

    //Mutable reference
    let mut s1 = String::from("hello");
    change(&mut s1);

    //Scope of a reference
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);  //Scope of r1 and r2 ends here

    let r3 = &mut s;
    println!("{}", r3);

    //Dangling reference
    // let reference_to_nothing = dangle();   This will give error due to invalid reference

    //Slices
    //Problem without slices
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();              //Return values is not tied with the string

    //Slices
    let mut s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    let word = first_word_with_slice(&s);
    //s.clear();             This will give error
    println!("the first word is: {}", word);

    let s2 = "hello world";
    let word = first_word_with_slice(s2);


}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length1(s: String) -> (String, usize){
    let length = s.len();
    (s, length)
}

fn calculate_length2(s: &String) -> usize{
    let length = s.len();
    length
}

fn change(some_string: &mut String){
    some_string.push_str(" world");
}

// fn dangle() -> &String{
//     let s = String::from("hello");
//     &s
// }

fn first_word(s: &String) -> usize{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_with_slice(s: &str) -> &str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}