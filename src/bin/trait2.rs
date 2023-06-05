// 導出アトリビュートでDebugトレイトを追加
// <T>(通常型):Typeの略、インスタンス化の際、任意の型を指定する事が可能
#[derive(Debug)]
struct Rect<T> {
    width: T,
    height: T,
}

trait Printable {
    fn print(&self);
    fn debug(&self);
}

// トレイトstd::fmt::Displayは追加することで{}で書きだすことが可能
// トレイトstd::fmt::Debugは追加することで{:?}で書きだすことが可能
impl<T> Printable for Rect<T>
where
    T: std::fmt::Display,
    T: std::fmt::Debug,
{
    fn print(self: &Rect<T>) {
        println!("Display: {}x{}", self.width, self.height);
    }
    fn debug(self: &Rect<T>) {
        println!("Debug: {:#?}", self);
    }
}

fn main() {
    let r1: Rect<i32> = Rect {
        width: 100,
        height: 200,
    };
    let r2: Rect<i64> = Rect {
        width: 100,
        height: 200,
    };
    r1.print();
    r1.debug();
    r2.print();
    r2.debug();
}
