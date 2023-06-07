use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    for _i in 1..10 {
        println!("{}", rng.gen_range(1..101));
    }
}
