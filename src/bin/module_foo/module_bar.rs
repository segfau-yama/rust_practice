pub fn bar_func() {
    // ルートモジュールのfooのbarのbar_hello()
    crate::module_foo::module_bar::bar_hello();
    // 親モジュールのbarのbar_hello()
    super::module_bar::bar_hello();
    // 自モジュールのbar_hello()
    self::bar_hello();

    println!("Bar!");
}
pub fn bar_hello() {
    println!("Hello!");
}
