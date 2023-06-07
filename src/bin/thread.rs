use std::thread;
use std::time::Duration;

fn main() {
    // thread::spawnでスレッド作成
    // 引数にクロージャー(ラムダ関数)を指定
    let th = thread::spawn(|| {
        for _i in 1..10 {
            println!("A");
            thread::sleep(Duration::from_millis(100));
        }
    });
    // unwrap: エラーだった場合強制終了
    th.join().unwrap();
    println!("Finished");

    let str = String::from("ABC");
    // moveで所有権を引き渡すことを明示
    let th = thread::spawn(move || {
        for _i in 1..10 {
            // strの所有権を得る
            println!("{}", str);
            thread::sleep(Duration::from_millis(100));
        }
    });
    th.join().unwrap();
    println!("Finished");
    // 所有権移動済のためエラー
    // println!("{}", str);
}
