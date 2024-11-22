fn main () {
    let x = 5;
    println!("The variable x is {}", x);
    let x = x + 1;
    println!("The variable x is {}", x);
}


// The difference between shadowing and mutabily:

// - Shadowing need let to reassign bind the variable name is a different value with mutable variable can have a different values by only assign it without let
// - Shadowing allow to give a different data type