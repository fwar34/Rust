//https://zhuanlan.zhihu.com/p/75755125
fn main() {
    let bird: FlyableBird = FlyableBird::new("test bird".to_string());
    eat_and_fly(&bird);
}

trait CanFly {
    fn fly(&self);
}

trait CanEat {
    fn eat(&self);
}

struct FlyableBird {
    name: String,
}

impl FlyableBird {
    pub fn new(name: String) -> FlyableBird {
        FlyableBird { name: name }
    }
}

impl CanFly for FlyableBird {
    fn fly(&self) {
        println!("{} eat", &self.name);
    }
}

impl CanEat for FlyableBird {
    fn eat(&self) {
        println!("{} fly", &self.name);
    }
}

fn eat_and_fly<T: CanFly + CanEat>(nth: &T) {
    nth.eat();
    nth.fly();
}
