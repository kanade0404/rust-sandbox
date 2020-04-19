fn main() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    // if number {
    //     println!("number was five");
    // }
    // error[E0308]: mismatched types
    //  --> src/main.rs:8:8
    //   |
    // 8 |     if number {
    //   |        ^^^^^^ expected `bool`, found integer
    // error: aborting due to previous error
    let num = 3;
    if number != 0 {
        println!("number {} was something other than zero", num);
    }
    println!("Handling Multiple Conditions with else if");

    let n = 6;
    if n % 4 == 0 {
        println!("number is divisible by 4");
    } else if n % 3 == 0 {
        println!("number is divisible by 3");
    } else if n % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is:{}", number);

    // let condition = true;
    // let number = if condition { 5 } else { "six" };
    // println!("The value of number is:{}", number);
    // error[E0308]: `if` and `else` have incompatible types
    // --> src/main.rs:39:44
    // |
    // 39 |     let number = if condition { 5 } else { "six" };
    // |                                 -          ^^^^^ expected integer, found `&str`
    // |                                 |
    // |                                 expected because of this

    // error: aborting due to previous error
    println!("Repetition with Loops");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFEOFF!!!");

    println!("Looping Through a Collection with for");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while 5 > index {
        println!("The value is:{}", a[index]);
        index += 1;
    }

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFEOFF!!!");
}
