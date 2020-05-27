fn main() {
    /*
    checked section
    */
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
    let y_r = y.clone(); // Box is not Copy

    println!("{}", y);
    println!("{}", y_r);

    /*
    Drop Flag section
    */
    let mut x = Box::new(0); // let makes a fresh variable, so never need to drop

    let y = &mut x;

    *y = Box::new(1); // Deref assumes the referent is initialized, so always drops

    // drop(x); //Error
    // println!("x: {}, y: {}", x, y); Error 

    /*
    Unchecked section
    */
    use std::mem::{self, MaybeUninit};

    // Size of the array is hard-coded but easy to change (meaning, changing just
    // the constant is sufficient). This means we can't use [a, b, c] syntax to
    // initialize the array, though, as we would have to keep that in sync
    // with `SIZE`!
    const SIZE: usize = 10;

    let x = {
        // Create an uninitialized array of `MaybeUninit`. The `assume_init` is
        // safe because the type we are claiming to have initialized here is a
        // bunch of `MaybeUninit`s, which do not require initialization.
        let mut x: [MaybeUninit<Box<u32>>; SIZE] = unsafe { MaybeUninit::uninit().assume_init() };

        // Dropping a `MaybeUninit` does nothing. Thus using raw pointer
        // assignment instead of `ptr::write` does not cause the old
        // uninitialized value to be dropped.
        // Exception safety is not a concern because Box can't panic
        for i in 0..SIZE {
            x[i] = MaybeUninit::new(Box::new(i as u32));
        }

        // Everything is initialized. Transmute the array to the
        // initialized type.
        unsafe { mem::transmute::<_, [Box<u32>; SIZE]>(x) }
    };

    dbg!(x);
}
