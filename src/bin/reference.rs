fn main() {
    let a = 123;
    // aのアドレスをpに代入する
    let p = &a;
    // pの参照するaの値を出力
    println!("pointer: {}, {:p}", *p, p);

    let a = 123;
    let ref p = a;
    println!("ref: {}, {:p}", *p, p);

    let mut a = 123;
    let p = &mut a;
    *p = 456;
    println!("mut pointer: {}, {:p}", a, &a);
}
