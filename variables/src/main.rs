fn main() {
    let mut x = 5;
    let s = String::from("hehe");

    println!("{s}");
    takes_ownership(s);

    let mut y = "hello";
    println!("The value of x is: {x}");
    let x = x + 1;

    {
        let x =  x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {y} {x}");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}