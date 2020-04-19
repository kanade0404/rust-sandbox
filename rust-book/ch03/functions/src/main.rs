fn main() {
    println!("Hello, world!");
    another_function();

    println!("Function Parameters");
    another_function1(5);

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of x is:{}", x);
    println!("The value of y is:{}", y);

    println!("Functions with Return Values");
    let x = five();
    println!("The value of x is:{}", x);
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn another_function() {
    println!("Another function.");
}
fn another_function1(x: i32) {
    println!("The value of x is:{}", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
