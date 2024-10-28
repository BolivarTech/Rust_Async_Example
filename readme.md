# Asynchronous Programming in Rust

Asynchronous programming in Rust allows you to write programs that can handle many tasks concurrently without being
blocked by any single task. This is particularly useful for I/O-bound and task-bound operations.

## Setting Up

To get started with asynchronous programming in Rust, you need to add some dependencies to your `Cargo.toml`:

```toml
[dependencies]
tokio = "1.41.0"
```

## Basic Concepts

### Futures

A future is a value that represents an asynchronous computation. A future does not do anything until you explicitly ask
it to do so. You can create a future using async blocks or functions.

### async/await syntax

Rust provides the `async` and `await` keywords to work with futures in a more ergonomic manner.

```rust
// Example of an async function
async fn example() -> u32 {
    42
}

// Calling the async function
#[tokio::main]
async fn main() {
    let result = example().await;
    println!("Result: {}", result);
}
```

### tokio

Tokio is an asynchronous runtime for Rust. It provides everything you need to run asynchronous tasks and effectively
handles I/O operations.

## Handling Concurrency

One of the powerful features of async programming is the ability to handle multiple tasks concurrently.

```rust
use tokio::time::{sleep, Duration};

async fn task_one() {
    println!("Task 1 starting.");
    sleep(Duration::from_secs(2)).await;
    println!("Task 1 done.");
}

async fn task_two() {
    println!("Task 2 starting.");
    sleep(Duration::from_secs(1)).await;
    println!("Task 2 done.");
}

#[tokio::main]
async fn main() {
    tokio::join!(task_one(), task_two());
}
```

In the above code, `task_one` and `task_two` run concurrently, even though `task_one` takes 2 seconds and `task_two`
takes 1 second, `tokio::join!` waits for both tasks to complete.

### `spawn_blocking`

Sometimes you may need to perform CPU-bound operations that should not run on the async runtime's worker threads, as
these operations could block the entire runtime. For such cases, Tokio provides the `spawn_blocking` function, which
allows you to offload these CPU-heavy tasks to a special thread pool dedicated to blocking operations.

Here's an example of how to use `spawn_blocking`:

```rust
use tokio::task;

#[tokio::main]
async fn main() {
    let result = task::spawn_blocking(|| {
        // This closure is run on a separate thread pool dedicated to blocking operations
        let mut sum = 0;
        for i in 1..=100 {
            sum += i;
        }
        sum
    }).await
    .expect("The task panicked");

    println!("The sum is: {}", result);
}
```

In this example:

- We use `task::spawn_blocking` to run a CPU-bound operation (a simple summation loop) on a separate thread pool.
- The closure inside `spawn_blocking` will be executed on a different thread than the async runtime's worker threads.
- We `await` the result of `spawn_blocking`, which gives us the outcome of the computation.
- It's important to handle the result of `spawn_blocking` with `.expect` or `.unwrap` to ensure any potential panic in
  the blocking task is properly handled.

Using `spawn_blocking` ensures that blocking operations do not interfere with the performance of the async runtime,
allowing your application to remain responsive while handling a mix of I/O-bound and CPU-bound tasks.

## This Repository

On this Repository are implemented async examples using Tokio spawn and spawn_blocking as references.

## Conclusion

Asynchronous programming is a powerful paradigm that, when used correctly, can greatly improve the performance and
scalability of your applications. Rust’s async/await syntax and libraries such as Tokio make it easier to write and
manage asynchronous code.