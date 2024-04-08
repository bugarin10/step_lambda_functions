use lambda_runtime::{run, service_fn, Error,LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    x: Vec<Vec<f64>>,
    y: Vec<i32>,
}

#[derive(Serialize)]
struct Response {
    x: Vec<Vec<f64>>,  // Changed to f64 to match the calculation in function_handler
    y: Vec<i32>,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error>  {
    // Standardize the X values
    let mut standardized_x = vec![];
    for row in event.payload.x.iter() {
        let mean: f64 = row.iter().sum::<f64>() as f64 / row.len() as f64;
        let std_dev: f64 = (row.iter().map(|xi| (xi - mean as f64).powf(2.0) as f64).sum::<f64>() / row.len() as f64).sqrt();
        let standardized_row: Vec<f64> = row.iter().map(|xi| ((xi - mean as f64) as f64 / std_dev)).collect();
        standardized_x.push(standardized_row);
    }

    Ok(Response {
        x: standardized_x,
        y: event.payload.y,
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
