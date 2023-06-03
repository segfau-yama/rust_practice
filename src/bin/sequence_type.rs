use std::collections::HashMap;

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

    let mut map = HashMap::new();
    map.insert("x", 10);
    map.insert("y", 20);
    map.insert("z", 30);
    println!("ハッシュマップ1: {} {} {}", map["x"], map["y"], map["z"]);
    println!("ハッシュマップ2:");
    for (k, v) in &map {
        println!("{} {}", k, v);
    }
}
