/// 新しくネコを追加しても、変更する必要のない既存のコード
trait Animal {
    fn name(&self) -> &str;
    fn greet(&self);
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }

    fn greet(&self) {
        println!("I'm {}.", self.name());
    }
}

fn main() {
    // 新しくネコを追加しても、変更する必要のない既存のコード
    let dog = Dog {
        name: "Kotetsu".to_string(),
    };
    dog.greet();

    // 新しく追加したネコを呼び出す、拡張したコード
    let cat = Cat {
        name: "Tama".to_string(),
    };
    cat.greet();
}

/// 拡張した実装
struct Cat {
    name: String,
}

impl Animal for Cat {
    fn name(&self) -> &str {
        &self.name
    }

    fn greet(&self) {
        println!("I'm {}.", self.name());
    }
}
