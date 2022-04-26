use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use std::hash::Hash;
use std::marker::Copy;

struct Cacher<'a, T, X, Y>
where
    X: Eq + Hash,
    Y: Copy,
    T: Fn(&X) -> Y,
{
    calculation: T,
    values: HashMap<&'a X, Y>,
}

impl<'a, T, X, Y> Cacher<'a, T, X, Y>
where
    X: Eq + Hash,
    Y: Copy,
    T: Fn(&X) -> Y,
{
    // fn new(calculation: T) -> Self {
    fn new(calculation: T) -> Cacher<'a, T, X, Y> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: &'a X) -> Y {
        if let Some(&v) = self.values.get(arg) {
            v
        } else {
            let value = (self.calculation)(&arg);
            self.values.insert(arg, value);
            value
        }
    }
}

fn generate_workout(intensity: &u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|&num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < &25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

// fn generate_workout_str() {
//     let mut expensive_result = Cacher::new(|message| {
//         println!("calculating slowly...");
//         thread::sleep(Duration::from_secs(2));
//         message.len()
//     });
// }

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(&simulated_user_specified_value, simulated_random_number);
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|&x| x);
    let _ = c.value(&1);
    let v2 = c.value(&2);
    assert_eq!(v2, 2);
}
