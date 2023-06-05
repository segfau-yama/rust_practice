fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn sub(x: i32, y: i32) -> i32 {
    x - y
}

// マクロはあまり利用されない
macro_rules! log {
    ($x:expr) => {
        println!("マクロ: {}", $x);
    };
}

fn main() {
    let x = 5;
    let y = 7;
    let mut z = add(x, y);
    println!("関数1: {}", z);

    z = sub(x, y);
    println!("関数2: {}", z);

    let square = |x: i32| x * x;
    println!("クロージャー1: {}", square(9));

    let msg = String::from("Hello");
    let func = move || {
        println!("クロージャー2: {}", msg);
    };
    func();
    // 解放済みなのでエラー
    // println!("{}", msg);

    log!("ABC...");
}
