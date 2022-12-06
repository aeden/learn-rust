fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("one");
    let s2 = s1.clone();
    println!("{s1} {s2}");

    takes_ownership(s1); // s1 moves into the function and cannot be used after this 

    let x = 5;
    makes_copy(x); // x would move into function, but i32 implements the Copy trait, so x can be used

    // example of function giving ownership of the return value
    let s3 = gives_ownership();
    println!("{s3}");

    // example of passing by value and returning value
    let s4 = takes_and_gives_back(s3);
    println!("After takes and gives back: {s4}");

    // example of passing by reference
    let s5 = String::from("five");
    let length = calculate_length(&s5);
    println!("The length of {s5} is {length}");

    // example of passing a mutable value to a function for mutation
    let mut s6 = String::from("six");
    change(&mut s6);
    println!("{s6}");

    println!("First word is at: {}", first_word(&String::from("Hello, world!")));
}

fn takes_ownership(s: String) { // s comes into scope
    println!("takes ownership: {s}");
} // s goes out of scope and `drop` is called, backing memory is freed

fn makes_copy(v: i32) { // v comes into scope
    println!("makes copy: {v}");
} // v goes out of scope, nothing special happens

fn gives_ownership() -> String {
    let s = String::from("two");
    println!("gives ownership: {s}");
    s
}

fn takes_and_gives_back(s: String) -> String {
    println!("takes and gives back {s}");
    s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" added");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}