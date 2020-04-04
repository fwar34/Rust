// https://zhuanlan.zhihu.com/p/78333162
fn main() {
    let mut flock = DuckFlock::new(33);
    {
        let thread: &mut dyn Thread = &mut flock;
        thread.kill(10);
    }

    {
        let flock: &mut dyn Flock = &mut flock;
        flock.kill(10);
    }

    {
        let thread: &mut dyn Thread = &mut flock;
        thread.kill(10);
    }
}

trait Thread {
    fn kill(&mut self, signal: i32);
}

trait Flock {
    fn kill(&mut self, amount: i32);
}

struct DuckFlock {
    ducks: i32,
}

impl DuckFlock {
    pub fn new(amount: i32) -> DuckFlock {
        DuckFlock { ducks: amount }
    }
}

impl Thread for DuckFlock {
    fn kill(&mut self, signal: i32) {
        if signal == 10 {
            println!("We have {} ducks", self.ducks);
        } else {
            println!("Unknow signal {}", signal);
        }
    }
}

impl Flock for DuckFlock {
    fn kill(&mut self, amount: i32) {
        self.ducks -= amount;
        println!("{} ducks killed!", amount);
    }
}
