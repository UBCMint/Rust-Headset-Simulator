use serde_json::{json, Value};
use rand::Rng;
use std::error::Error;

// Generate a random matrix and return it as a JSON string
fn generate_random_matrix() -> Result<String, Box<dyn Error>> {
    let rows = 20;
    let cols = 5;

    let mut rng = rand::thread_rng();
    let mut data = Vec::new();
    for _ in 0..rows {
        let mut row = Vec::new();
        for _ in 0..cols {
            row.push(rng.gen::<i32>()); 
        }
        data.push(row);
    }

    let result = json!({
        "metadata": {
            "size": format!("{}x{}", rows, cols)
        },
        "data": data
    });

    let result_string = serde_json::to_string(&result)?;

    Ok(result_string)
}
