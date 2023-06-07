mod module_foo;
use module_foo::module_bar;
// 子モジュールの呼び出し(名前変更)
use module_foo::module_bar as bbaarr;
// 子モジュールの呼び出し(子要素を全て使用可能に)
use module_foo::module_bar::*;

fn main() {
    module_foo::foo_func();

    module_bar::bar_func();
    bbaarr::bar_func();
    bar_func();
}
