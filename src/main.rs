use std::future::IntoFuture;
use std::time::Duration;
use rand::Rng;
use tokio::time::sleep;

/// The entry point of the application. This is an asynchronous main 
/// function that spawns and runs multiple asynchronous tasks concurrently.
#[tokio::main]
async fn main()
{
    let f = tokio::spawn(async_fn());
    let g = tokio::spawn(async_fn2());
    let h = tokio::spawn(long_cycle());
    println!("From Main!!!");
    h.await.unwrap();
    f.await.unwrap();
    g.await.unwrap();
}

/// An example asynchronous function that simulates reading from a database.
///
/// This function prints a message, reads a value from the database asynchronously, and
/// then prints the result.
async fn async_fn()
{
    println!("Hello, async_fn!");
    let s1 = read_from_db("Async 1").await;
    println!("First Result = {}", s1);
}

/// Another example asynchronous function that simulates reading from a database.
///
/// This function prints a message, reads a value from the database asynchronously, and
/// then prints the result.
async fn async_fn2()
{
    println!("Hello, async_fn2!");
    let s2 = read_from_db("Async 2").await;
    println!("Second Result = {}", s2);
}

/// Simulates reading data from a database with random delay.
///
/// This function generates a random number of seconds to delay, awaits for that duration,
/// and then returns a formatted string indicating the source of the read.
///
/// # Arguments
///
/// * `a` - A string slice that holds the name of the source.
///
/// # Returns
///
/// * `String` - A formatted string indicating which source was read.
async fn read_from_db(a: &str) -> String
{
    let num = rand::thread_rng().gen_range(0..10);
    sleep(Duration::from_secs(num)).await;
    format!("DB Read from {a}").to_owned()
}

/// A structure that holds the result of prime number verification.
struct PrimeVerification {
    value: u32,
    is_prime: bool
}

/// A long-running asynchronous function that calculates prime numbers.
///
/// This function spawns blocking tasks to determine if numbers are prime, collects
/// the prime numbers into a vector, and prints them.
async fn long_cycle()
{
    let find_range: u32 = 1_000_000;
    let num_task = 8;
    let mut primes: Vec<u32> = Vec::new();
    let mut handles = Vec::new();
    let mut value: u32 = 0;

    println!("Long Calc Started!");
    loop {
        while (handles.len() <= num_task) && (value <= find_range)
        {
            let result = tokio::task::spawn_blocking(move || { is_prime(value) });
            handles.push(result);
            value += 1;
        }
        loop
        {
            for index in 0..handles.len()
            {
                if let Some(handle) = handles.get(index) {
                    if !handle.is_finished() {
                        continue;
                    }
                    let finished_handle = handles.remove(index);
                    if let Ok(result) = finished_handle.into_future().await {
                        if result.is_prime {
                            primes.push(result.value);
                        }
                    }
                }
            }
            if handles.len() < num_task
            {
                break;
            }
        }
        if handles.is_empty() && (value > find_range)
        {
            break;
        }
    }
    println!("Prime Numbers");
    for i in primes
    {
        print!("{i},");
    }
    println!("\x08 ");
    println!("Long Calc Finished!");
}

/// Determines if a number is prime.
///
/// This function checks if the given number is prime by testing for divisibility
/// from 2 to the square root of the number.
///
/// # Arguments
///
/// * `n` - The number to be checked.
///
/// # Returns
///
/// * `PrimeVerification` - A struct containing the value and its primality.
fn is_prime(n: u32) -> PrimeVerification {
    let mut respond = PrimeVerification {
        value: n,
        is_prime: false,
    };
    if respond.value <= 1 {
        return respond;
    }
    for a in 2..=((n as f64).sqrt() as u32) {
        if respond.value % a == 0 {
            return respond; // Return immediately if a divisor is found
        }
    }
    respond.is_prime = true; // If no divisors are found, the number is prime
    respond
}

#[cfg(test)]
mod tests
{
    use super::*;

    /// Tests the prime number detection logic.
    #[test]
    fn prime_number_test()
    {
        let primes: [u32; 25] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 
                                 61, 67, 71, 73, 79, 83, 89, 97];
        let mut calc_primes: Vec<u32> = Vec::new();
        for i in 0..101u32
        {
            let result = is_prime(i);
            if result.is_prime
            {
                calc_primes.push(i);
            }
        }
        assert_eq!(calc_primes.len(), primes.len(), "Prime Vector not have the same length");
        for i in 0..primes.len()
        {
            assert_eq!(calc_primes[i], primes[i], "Prime number {} not found",calc_primes[i]);
        }
    }
}