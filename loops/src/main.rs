fn main() {
    // Countdown

    let mut x:u8 = 10;


    println!("Counting down from {} with loops", x);
    loop {
        if x == 0 {
            println!("Number {}", x);
            break;
        } else {
            println!("Number {}", x);
            x -= 1;
        }
    }

    println!("Counting down from {} with FOR LOOPS", x);
    
    let mut y:u8 = 10;

    for number in (0..=y).rev() {
        if number == 0 {
            println!("Printing {}", y);
            break;
        } else {
        println!("Printing {}", y);
        y-= 1
        }
    }

    println!("Counting down from {} with WHILE LOOPS", x);

    let mut z: u8 = 10;
    while z > 0 {
        println!("Printing {}",z);
        z-=1;

    }

}
