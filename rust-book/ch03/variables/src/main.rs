fn main() {
    let mut x = 5;
    println!("The value of x is:{}", x);
    x = 6;
    println!("The value of x is:{}", x);

    // Build Error
    // let x = 5;
    // println!("The value of x:{}", x);
    // x = 6;
    // println!("The value of x")
    //     error[E0384]: cannot assign twice to immutable variable `x`
    //  --> src/main.rs:4:5
    //   |
    // 2 |     let x = 5;
    //   |         -
    //   |         |
    //   |         first assignment to `x`
    //   |         help: make this binding mutable: `mut x`
    // 3 |     println!("The value of x:{}", x);
    // 4 |     x = 6;
    //   |     ^^^^^ cannot assign twice to immutable variable

    println!("Shadowing.");
    let a = 5;
    let a = a + 1;
    let a = a * 2;
    println!("The value of a is :{}", a);
}
