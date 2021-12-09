use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly---");
    thread::sleep(Duration::from_secs(2));
    intensity
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T> where
    T: Fn(u32) -> u32, {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new()
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        if self.value.contains_key(&arg) {
            *self.value.get(&arg).unwrap()
        } else {
            self.value.insert(arg, (self.calculation)(arg));
            *self.value.get(&arg).unwrap()
        }

    }
}

pub fn generate_workout(intensity: u32, random_number: u32) {

    let mut expensive_result = Cacher::new( |num|{
        println!("calculating slowly---");
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
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

#[test]
fn cacher_test() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);
    let v9 = c.value(2);
    let v3 = c.value(3);
    let v5 = c.value(5);
    let v8 = c.value(8);
    let v7 = c.value(7);

    assert_eq!(v2, 2);
    assert_eq!(v2, 2);
    assert_eq!(v9, 2);
    assert_eq!(v3, 3);
    assert_eq!(v5, 5);
    assert_eq!(v7, 7);
}