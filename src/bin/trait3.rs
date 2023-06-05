use std::boxed::Box;

struct Dog {}
struct Cat {}
trait Animal {
    fn cry(&self);
}

impl Animal for Dog {
    fn cry(&self) {
        println!("Bow-wow");
    }
}
impl Animal for Cat {
    fn cry(&self) {
        println!("Miaow");
    }
}

// dyn:トレイトオブジェクトの明示
// Boxを利用し、Animalトレイトを戻り値にする
fn get_animal(animal_type: &str) -> Box<dyn Animal> {
    if animal_type == "dog" {
        return Box::new(Dog {});
    } else {
        return Box::new(Cat {});
    }
}

fn main() {
    get_animal("dog").cry();
    get_animal("cat").cry();
}
