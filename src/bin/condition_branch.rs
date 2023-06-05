fn main() {
    let n = 4;
    print!("if1: ");
    if n == 1 {
        println!("One");
    } else if n == 2 {
        println!("Two");
    } else {
        println!("other");
    }

    let s = if n == 1 { "OK!" } else { "NG!" };
    println!("if2: {}", s);

    let x = 2;
    match x {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("other"),
    }
}
