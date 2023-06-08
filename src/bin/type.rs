fn main() {
    // 型エイリアスにより型に命名する
    type Meter = u8;
    type Millimeter = u32;
    let m: Meter = 12;
    let mm: Millimeter = 12000;
    println!("{} {}", m, mm);

    let x = 123;
    println!("{}", type_of(x));
}

// ジェネリック型(T)にtypeを格納, 値は_に格納
fn type_of<T>(_: T) -> &'static str {
    // type_nameで<T>の中の型名を返す。
    // 戻り値は静的なstr型に格納
    std::any::type_name::<T>()
}
