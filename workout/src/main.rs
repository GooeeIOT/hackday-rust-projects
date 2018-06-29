use std::thread;
use std::time::Duration;

fn main() {
    let intensity = 22;
    let random_number = 3;
    generate_workout(intensity, random_number);
}

struct Cacher<T, U>
where
    T: Fn(U) -> U,
{
    calculation: T,
    value: Option<U>,
}

impl<T, U> Cacher<T, U>
where
    T: Fn(U) -> U,
    U: Eq + Copy,
{
    fn new(calculation: T) -> Cacher<T, U> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: U) -> U {
        match self.value {
            Some(v) if v == arg => v,
            _ => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
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
    let mut c = Cacher::new(|a| a);

    let _ = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
