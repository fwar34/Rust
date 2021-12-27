// https://zhuanlan.zhihu.com/p/78333162
fn main() {
    eat(PeikingDuck);
    // eat(Liver); //compile error
}

trait TonyFavorite {}
trait Food {
    fn name(&self) -> String;
}

struct PeikingDuck;

impl Food for PeikingDuck {
    fn name(&self) -> String {
        "Peiking Duck".to_owned()
    }
}

impl TonyFavorite for PeikingDuck {}

struct Liver;

impl Food for Liver {
    fn name(&self) -> String {
        "Liver".to_owned()
    }
}

fn eat<T: Food + TonyFavorite>(food: T) {
    println!("Tony only eat hist favorite food like {}", food.name());
}
