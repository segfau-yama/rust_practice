struct Rect {
    width: u32,
    height: u32,
}

trait Printable {
    fn print(&self);
}
impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Printable for Rect {
    fn print(&self) {
        println!("トレイト: width:{}, height:{}", self.width, self.height);
    }
}

fn main() {
    let r = Rect {
        width: 200,
        height: 300,
    };
    println!("インプリメンテーション: {}", r.area());
    r.print();
}
