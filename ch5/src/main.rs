fn main() {
    let x: i32;

    if true {
        x = 5;
    } else {
        x = 7;
    }
    println!("{}", x);

    let x_r = x;
    println!("{}", x);
    println!("{}", x_r);

    let y = Box::new(1);
    let y_r = y.clone();

    println!("{}", y);
    println!("{}", y_r);
}
