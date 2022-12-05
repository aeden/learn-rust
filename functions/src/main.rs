fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("Value of y: {y}");
    println!("Value of f: {}", f(20));
}

fn f(a: i32) -> i32 {
    a + 10
}
