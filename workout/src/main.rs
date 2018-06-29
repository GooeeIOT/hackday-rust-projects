use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

fn main() {
    let intensity = 22;
    let random_number = 3;
    generate_workout(intensity, random_number);
}

struct Cacher<T, U, V>
where
    T: Fn(&U) -> V,
{
    calculation: T,
    values: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
where
    T: Fn(&U) -> V,
    U: Eq + Hash,
    V: Copy,
{
    fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V {
        match self.values.get(&arg) {
            Some(&v) => v,
            None => {
                let v = (self.calculation)(&arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        *num
    });

    if intensity < 25 {
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

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| *a);

    let _ = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
