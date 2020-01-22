use std::thread;
use std::time::Duration;
use std::collections::HashMap;

struct Cacher<T, U, V>
    where T: Fn(U) -> V,
          U: Copy + std::cmp::Eq + std::hash::Hash,
          V: Copy
{
    calculation: T,
    value: HashMap<U, V>
}

impl<T, U, V> Cacher<T, U, V>
    where T: Fn(U) -> V,
          U: Copy + std::cmp::Eq + std::hash::Hash,
          V: Copy
{
    fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            value: HashMap::new(),

        }
    }

    fn value(&mut self, arg: U) -> V {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            },
        }
    }
}


fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else  {
        if random_number == 3 {
            println!(
                "Take a break today! Remember to stay hydrated!"
            );
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

    let _v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

#[test]
fn call_with_different_types() {
    let mut c = Cacher::new(|a: &str|  a.len());

    let _v1 = c.value("1st slice");
    let v2 = c.value("second slice");

    assert_eq!("second slice".len(), v2);
}
