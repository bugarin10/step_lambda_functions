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
    // Get the number of columns
    let num_cols = event.payload.x[0].len();

    // Initialize vectors to hold the mean and standard deviation for each column
    let mut means = vec![0.0; num_cols];
    let mut std_devs = vec![0.0; num_cols];

    // Calculate the mean for each column
    for row in &event.payload.x {
        for (j, xi) in row.iter().enumerate() {
            means[j] += xi;
        }
    }
    for mean in &mut means {
        *mean /= event.payload.x.len() as f64;
    }

    // Calculate the standard deviation for each column
    for row in &event.payload.x {
        for (j, xi) in row.iter().enumerate() {
            std_devs[j] += (xi - means[j]).powf(2.0);
        }
    }
    for std_dev in &mut std_devs {
        *std_dev = (*std_dev / event.payload.x.len() as f64).sqrt();
    }

    // Standardize the X values
    let mut standardized_x = vec![];
    for row in &event.payload.x {
        let standardized_row: Vec<f64> = row.iter().enumerate().map(|(j, xi)| (xi - means[j]) / std_devs[j]).collect();
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
