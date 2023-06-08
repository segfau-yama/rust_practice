extern "C" {
    // C言語のabs関数を定義
    fn abs(x: i32) -> u32;
}

fn main() {
    unsafe {
        println!("{}", abs(-123));
    }
}
