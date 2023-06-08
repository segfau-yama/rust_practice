static mut COUNTER: u32 = 0;

fn func() {
    unsafe {
        // static型なので複数スレッドから参照可能
        COUNTER += 1;
    }
}
fn main() {
    unsafe {
        println!("{}", COUNTER);
        func();
        println!("{}", COUNTER);
    }
}
