fn main() {
    let mut name: &str = "Yamada";
    println!("文字列1(&str):{}", name);
    name = "Tanaka";
    println!("文字列2(&str):{}", name);

    let mut name = String::from("Yamada");
    println!("文字列1(String):{}", name);
    name = "Tanaka".to_string();
    name.push_str(" Taro");
    println!("文字列2(String):{}", name);
}
