

fn main() {
    exercise1(); 
    exercise3();
    exercise4();
    exercise5();
}

fn exercise1() {
    // TODO: Change the line below to fix the compiler error.
    let x:i32 = 12;

    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}


fn exercise3() {
    let x: i32 = 32;

    println!("Number {x}");
}

fn exercise4(){
    let mut x = 3;
    println!("Number {x}");

    x = 5;
    println!("Number {x}");
}

fn exercise5(){
    let number = "T-H-R-E-E";
    println!("Spell a number: {}", number);

    let number = 3;
    println!("Number plus two is: {}", number + 2);
}

