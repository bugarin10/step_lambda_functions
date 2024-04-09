use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    x: Vec<Vec<f64>>,
    y: Vec<i32>,
}

#[derive(Serialize)]
struct Response {
    x_train: Vec<Vec<f64>>,
    x_test: Vec<Vec<f64>>,
    y_train: Vec<i32>,
    y_test: Vec<i32>,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let request = event.payload;

    let Request { x, y } = request;

    // Combine x and y into tuples for shuffling
    let mut combined: Vec<_> = x.clone().into_iter().zip(y.clone().into_iter()).collect();
    combined.shuffle(&mut rand::thread_rng());

      // Determine the split point based on 70% training and 30% test
      let split_point = (x.len() as f64 * 0.7) as usize;

      // Split the x and y data into training and test sets
      let x_train = x[..split_point].to_vec();
      let x_test = x[split_point..].to_vec();
      let y_train = y[..split_point].to_vec();
      let y_test = y[split_point..].to_vec();

    Ok(Response {
        x_train,
        x_test,
        y_train,
        y_test,
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
