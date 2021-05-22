use std::thread;
use std::{collections::HashMap, hash::Hash, time::Duration};
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("Calculateing slowly...");
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

struct Cacher<T, U>
where
    T: Fn(U) -> U,
    U: Copy + Eq + Hash,
{
    calculation: T,
    value: HashMap<U, U>,
}
impl<T, U> Cacher<T, U>
where
    T: Fn(U) -> U,
    U: Copy + Eq + Hash,
{
    fn new(calculation: T) -> Cacher<T, U> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }
    fn value(&mut self, args: U) -> U {
        match self.value.get(&args) {
            Some(value) => *value,
            None => {
                let c = (self.calculation)(args);
                self.value.insert(args, c);
                c
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cacher() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
        assert_eq!(v1, 1);
    }
}
