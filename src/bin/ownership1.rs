/*
fn func1() {
    // nameがStringの所有権を持つ
    let name = String::from("ABC");
    println!("{}", name);
}
// nameがスコープアウトしたので解放される
*/

/*
fn func1() {
    let name = String::from("ABC");
    println!("{}", name);
    // ここで所有権がfunc2()のnameに移動してしまう
    func2(name);
    // func2()終了時に開放済の領域を参照しているのでエラー
    println!("{}", name);
}

fn func2(name: String) {
    // func1()から所有権を奪い取る
    println!("{}", name);
}
// この時点でヒープ領域が解放されてしまう
*/

fn func1() {
    let mut name = String::from("ABC");
    println!("{}", name);
    // 所有権を渡した後、返却してもらう
    name = func2(name);
    println!("{}", name);
}

fn func2(name: String) -> String {
    println!("{}", name);
    // 所有権を返却する
    name
}

fn main() {
    func1();
}
