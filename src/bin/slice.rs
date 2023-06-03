fn main() {
    let s = String::from("ABCDEFG");
    // 0から2までスライス
    let s1 = &s[0..3];
    // 3から5までスライス
    let s2 = &s[3..6];
    println!("{} {}", s1, s2);
    let a = [10, 20, 30, 40, 50, 60, 70, 80];
    let a1 = &a[0..3];
    let a2 = &a[3..6];
    println!("{:?} {:?}", a1, a2);
}
