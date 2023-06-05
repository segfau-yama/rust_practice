struct Counter {
    max: u32,
    count: u32,
}

// newで最大値を指定したCounter構造体を作成
impl Counter {
    fn new(max: u32) -> Counter {
        Counter { max: max, count: 0 }
    }
}

// Counterにforで利用できるイテレータトレイトを実装
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < self.max {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let counter = Counter::new(10);
    for c in counter {
        println!("{}", c);
    }
}
