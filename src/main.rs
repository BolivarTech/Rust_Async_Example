use std::future::IntoFuture;
use std::time::Duration;
use rand::Rng;
use tokio::time::sleep;

#[tokio::main]
async fn main()
{
    let f = tokio::spawn(async_fn());
    let g = tokio::spawn(async_fn2());
    let h = tokio::spawn(long_cycle());
    /*
    let h = tokio::spawn( async move {
    let _ = tokio::task::spawn_blocking(|| { long_cycle() }).await;
    });
    */
    println!("From Main!!!");
    h.await.unwrap();
    f.await.unwrap();
    g.await.unwrap();
}

async fn async_fn()
{
    println!("Hello, async_fn!");
    let s1 = read_from_db("Async 1").await;
    println!("First Result = {}", s1);
}

async fn async_fn2()
{
    println!("Hello, async_fn2!");
    let s2 = read_from_db("Async 2").await;
    println!("Second Result = {}", s2);
}

async fn read_from_db(a: &str) -> String
{
    let num = rand::thread_rng().gen_range(0..10);
    sleep(Duration::from_secs(num)).await;
    format!("DB Read from {a}").to_owned()
}

struct PrimeVeryfication {
    value: u32,
    is_prime: bool
}

async fn long_cycle()
{
    let num_taks = 8;
    let mut primes: Vec<u32> = Vec::new();
    let mut handles = Vec::new();

    for i in 2..1_000
    {
        if handles.len() < num_taks
        {
            let result = tokio::task::spawn_blocking(move ||{is_prime(i)});
            handles.push(result);
        } 
        else 
        {
            let mut temphdl= Vec::new();
            for handle  in handles.drain(0..num_taks)
            {
                if handle.is_finished() {
                    let result = handle.into_future().await.unwrap();
                    if result.is_prime {
                        primes.push(result.value);
                    }
                } 
                else 
                {
                    temphdl.push(handle);
                    //sleep(Duration::from_millis(5)).await; 
                }
            }
            handles = temphdl;

        }
    }
    
    println!("Long Calc Finished!");
    println!("Prime Numbers");
    for i in primes
    {
        print!("{i},");
    }
    println!("\x08 ");
}

fn is_prime(n: u32) -> PrimeVeryfication {
    let mut respond = PrimeVeryfication {
        value: n,
        is_prime: false,
    };
    if respond.value <= 1 {
        return respond;
    }
    for a in 2..n {
        if respond.value % a == 0 {
            return respond; // if it is not the last statement you need to use `return`
        }
    }
    respond.is_prime = true; // last value to return
    respond
}