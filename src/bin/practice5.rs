struct Point {
    x: i32,
    y: i32,
}

impl Drop for Point {
    fn drop(&mut self) {
        println!("メモリ解放");
    }
}

fn main() {
    // Boxスマートポインタ、ヒープ領域のメモリを確保
    let p: Box<Point> = Box::new(Point { x: 100, y: 200 });
    println!("{} {}", p.x, p.y);
}
