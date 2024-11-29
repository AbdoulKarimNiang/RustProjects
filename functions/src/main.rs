fn main() {
    println!("Hello, world!");
    another_function();
    print_measurent(50, 'l');

}

fn another_function(){
    print!("Another function")
}

fn print_measurent(x: i32, y: char) {
    println!("The measurement is {x} {y}");
}