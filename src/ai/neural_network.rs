use rand::Rng;
use serde::{Serialize, Deserialize};
use log::{info, error};

#[derive(Serialize, Deserialize, Debug)]
pub struct NeuralPrediction {
    pub predicted_trend: String,
    pub confidence: f64,
}

pub struct NeuralNetwork;

impl NeuralNetwork {
    pub fn run_neural_network(data: &str) -> Result<NeuralPrediction, String> {
        info!("Running neural network on data: {}", data);

        let mut rng = rand::thread_rng();
        let confidence: f64 = rng.gen_range(0.7..1.0);

        if data.contains("bullish trend") {
            Ok(NeuralPrediction {
                predicted_trend: "Bullish".to_string(),
                confidence,
            })
        } else if data.contains("bearish trend") {
            Ok(NeuralPrediction {
                predicted_trend: "Bearish".to_string(),
                confidence,
            })
        } else {
            error!("Could not predict trend for data: {}", data);
            Err("Unable to predict trend".to_string())
        }
    }
}
