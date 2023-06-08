fn func1() {
    let name = String::from("ABC");
    println!("{}", name);
    // 参照のみを渡して所有権は渡さない
    func2(&name);
    // 所有権が残っているので参照可能
    println!("{}", name);
}

// func1()から参照のみを借用する
fn func2(name: &String) {
    println!("{}", name);
}
// 参照のみなのでヒープ領域は解放されない

/*
fn func1() {
    let s1 = String::from("ABC");
    {
        let s2 = s1; // 所有権がs1からs2に移動
        println!("{}", s2);
    } // 所有者が居なくなるので解放される
    println!("{}", s1); // エラー
}
*/

fn main() {
    func1();
}
