use rand::Rng;



#[allow(dead_code)]
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
}

#[allow(dead_code)]
pub fn activation_relu(input: f32) -> f32 {
    input.max(0.0)
}

#[allow(dead_code)]
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