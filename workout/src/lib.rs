use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

pub struct Cacher<'a, F, A: 'a, R: 'a> {
    calculation: F,
    values: HashMap<&'a A, &'a R>,
}

impl<'a, F, A, R> Cacher<'a, F, A, R>
where
    F: Fn(&A) -> &R,
    A: Eq + Hash,
{
    pub fn new(calculation: F) -> Cacher<'a, F, A, R> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: &'a A) -> &'a R {
        match self.values.get(&arg) {
            Some(&v) => &v,
            None => {
                let v = (self.calculation)(&arg);
                self.values.insert(&arg, v);
                &v
            }
        }
    }
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(&intensity));
        println!("Next, do {} situps!", expensive_result.value(&intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(&intensity)
            );
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|_| &"hello");

    let _ = c.value(&3);
    let v2 = c.value(&3);

    assert_eq!(v2, &"hello");
}
