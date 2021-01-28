use std::thread;
use std::time::Duration;

// Struct to hold a fn that takes a u32 and
// returns a u32
// Note: Holds a value of Option<u32> as this holds
// the return value of the fn but can also hold None
// (For when we initialize but haven't executed the code)
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    // when we now create closures using the Cacher struct
    // we store the code to execute as well an an
    // initial return value of None (Returning a Cacher instance)
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    // When we call value on the closure we store in Cacher
    // we either return an existing value generated by an earlier
    // closure execution or we execute, store and return the value
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

pub fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    // A closure
    // Contains the definition of a function stored to be
    // used later
    // Note: Closures don't require type annotations
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!")
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

fn closure_usage_example() {
    let example_closure = |x| x;

    // Compiles and the inference is that the closure
    // above accepts string
    let s = example_closure(String::from("hello"));

    // Doesn't compile as closure is already inferred to string
    // let n = example_closure(5);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    // This test fails for valid reasons
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        // Once we set the Cacher closure with a value the
        // Cacher logic will return the value instead of
        // re-executing the code so v2 is never set to 2
        // and instead has the cached value 1 returned
        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }

    /*
    #[test]
    // This test doesn't compile or valid reasons
    fn closure_environment_test() {
        let x = 4;

        // Environment values used in a closure are valid!
        let equal_to_x_closure = |z| z == x;

        // Won't compile:
        // 'can't capture dynamic environment in a fn item'
        // i.e. can't compare external environment variables in function
        fn equal_to_x_fn(z: i32) -> bool {
          z == x
        }

        let z = 4;

        assert!(equal_to_x_closure(z));
        assert!(!equal_to_x_fn(z))
    }
     */


    /*
    #[test]
    // this test demonstrates the move of a environment variables
    // to within a closure
    // As we move the variables and the closure takes ownership of x
    // it can't be borrowed by println
    // Note: vec type doesn't implement copy otherwise move would perform that
    fn closure_move_example() {
        let x = vec![1, 2, 3];

        let equal_to_x = move |z| z == x;

        // test won't compile because of this line
        println!("Can't use x here: {:?}", x);

        let y = vec![1, 2, 3];

        assert!(equal_to_x(y));
    }
     */
}