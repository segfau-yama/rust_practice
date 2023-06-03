struct Point {
    x: i32,
    y: i32,
}

union MyUnion {
    f1: u32,
    f2: u32,
}

enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let p = Point { x: 100, y: 200 };
    println!("構造体: {} {}", p.x, p.y);

    let u = MyUnion { f1: 123 };
    // 共用体はスレッドセーフではない
    unsafe {
        println!("共用体: {} {}", u.f1, u.f2);
    }

    // 要素がColor型のベクタを作成
    let colors: Vec<Color> = vec![Color::Red, Color::Green, Color::Blue];
    println!("列挙型:");
    for color in colors {
        print!("  ");
        match color {
            Color::Red => println!("Red"),
            Color::Green => println!("Green"),
            Color::Blue => println!("Blue"),
        }
    }
    println!("");
}
