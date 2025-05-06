use rand::Rng;
use rand_distr::{Distribution, Normal};
use serde::{Serialize, Deserialize};

use std::fs::{self, File};
use std::io::Write;
use bincode;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone)]
pub struct NeuralNetwork {
    pub layers: Vec<Layer>,
}

impl NeuralNetwork {
    #[allow(dead_code)]
    pub fn new(layer_sizes: Vec<usize>) -> Self {
        let mut network = Self { layers: Vec::new() };
        
        for n in 1..layer_sizes.len() {
            network.layers.push(Layer::create_layer(layer_sizes[n], layer_sizes[n - 1]));
        }
        network
    }

    #[allow(dead_code)]
    pub fn evaluate(&self, input: Vec<f32>) -> Vec<f32> {
        let mut output = input;

        //Loop through all layers
        for layer in &self.layers {
            output = layer.forward_layer(output);
        }

        output
    }

    #[allow(dead_code)]
    pub fn mutate(&self, mutation_rate: f32, change: f32) -> Self {
        let mut clone = self.clone();

        let mut rng = rand::rng();
        let normal = Normal::new(0.0, change).unwrap();

        for layer in &mut clone.layers {
            for (i, node) in layer.weights.iter_mut().enumerate() {
                for weight in node {
                    if rng.random_range(0.0..1.0) < mutation_rate {
                        *weight += normal.sample(&mut rng) as f32;
                    }
                }
                layer.biases[i] += normal.sample(&mut rng) as f32;
            }
        }

        clone
    }

    #[allow(dead_code)]
    pub fn save(&self, path: &str) {
        let encoded = bincode::serialize(self).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(&encoded).unwrap();
    }
    
    #[allow(dead_code)]
    pub fn load_network(path: &str) -> Self {
        let data = fs::read(path).unwrap();
        bincode::deserialize(&data).unwrap()
    }
}

#[allow(dead_code)]
pub fn activation_relu(input: f32) -> f32 {
    input.max(0.0)
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone)]
pub struct Layer {
    pub weights: Vec<Vec<f32>>,
    pub biases: Vec<f32>,
}

impl Layer {
    #[allow(dead_code)]
    pub fn create_layer(layer_nodes: usize, previous_layer_nodes: usize) -> Self {
        let mut rng = rand::rng();

        let mut layer = Self { weights: Vec::new(), biases: Vec::new() };

        for i in 0..layer_nodes {
            layer.biases.push(rng.random_range(-1.0..1.0));
            layer.weights.push(Vec::new());
            for _ in 0..previous_layer_nodes {
                layer.weights[i].push(rng.random_range(-2.0..2.0) * (2.0 / previous_layer_nodes as f32).sqrt());
            }
        }

        layer
    }

    pub fn forward_layer(&self, input: Vec<f32>) -> Vec<f32> {
        let mut output = Vec::with_capacity(input.len());

        for node in 0..self.biases.len() {
            let mut out_val = self.biases[node];

            for (i, n) in self.weights[node].iter().enumerate() {
                out_val += n * input[i];
            }

            output.push(activation_relu(out_val));
        }

        output
    }
}