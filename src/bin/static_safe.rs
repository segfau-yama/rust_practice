// 原子性を保証する型(Atomic)でCOUNTERを定義
use std::sync::atomic::{self, AtomicU32, Ordering};
static COUNTER: AtomicU32 = AtomicU32::new(0);

fn count_up() {
    // SeqCst:メモリ順序最強のメソッド
    COUNTER.fetch_add(1, atomic::Ordering::SeqCst);
}
fn get_count() -> u32 {
    return COUNTER.load(Ordering::SeqCst);
}

fn main() {
    println!("{}", get_count());
    count_up();
    println!("{}", get_count());
}
