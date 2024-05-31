fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    let mut s = String::from("hello");

    let r1 = &s; // no problem, only borrowing
    let r2 = &s; // no problem, only barrowing
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem, mut variable after others are finished being used
    println!("{}", r3);


    //Dangling Refs
    //    let reference_to_nothing = dangle();


    //Slices
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5
 
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
    
    // fn dangle() -> &String {
    //     let s = String::from("hello"); //s initialised
    
    //     &s
    // } //s goes out of scope

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}