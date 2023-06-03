fn main() {
    let tup = (10, "20", 3.3);
    println!("タプル: {} {} {}", tup.0, tup.1, tup.2);

    let arr = [10, 20, 30];
    println!("配列1: {} {} {}", arr[0], arr[1], arr[2]);

    println!("配列2:");
    for v in &arr {
        println!("  {}", v)
    }

    let mut vect = vec![10, 20, 30];
    vect.push(40);
    println!("ベクタ1: {} {} {} {}", vect[0], vect[1], vect[2], vect[3]);
    println!("ベクタ2:");
    for v in &vect {
        println!("  {}", v);
    }
}
