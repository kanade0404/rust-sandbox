fn main() {
    println!("Integer Types");
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Hello, world! {}", guess);

    println!("Floating-Point Types");
    let x = 2.0;
    let y: f32 = 3.0;
    println!("{}, {}", x, y);

    println!("Numeric Operations");
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quoinent = 56.7 / 32.2;
    let remainder = 43 % 5;
    println!(
        "sum:{}, difference:{}, product:{}, quoinent:{}, remainder:{}",
        sum, difference, product, quoinent, remainder
    );

    println!("The Boolean Type");
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("{}, {}", t, f);

    println!("The Character Type");
    let c = 'z';
    let z = 'ℤ';
    let heart_eyes_cat = '😻';
    println!("c:{}, z:{}, heart_eyes_cat:{}", c, z, heart_eyes_cat);

    println!("Compoud Types");
    println!("Tuple");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (i, f, u) = tup;
    println!("i:{}, f:{}, u:{}", i, f, u);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!(
        "five_hundred:{}, six_point_four:{}, one:{}",
        five_hundred, six_point_four, one
    );

    println!("The Array Type");
    let a = [1, 2, 3, 4];
    let month = ["January", "February"];
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = month[1];
    let three_index = 2;
    let third = arr[three_index];
    println!("first:{}, second:{}, third:{}", first, second, third);
}
