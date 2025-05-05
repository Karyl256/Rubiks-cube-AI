mod cube;
mod cube_rotations;
mod neural_network;

use neural_network::NeuralNetwork;
use cube::Cube;

fn main() {
    let network = NeuralNetwork::new(vec![324, 40, 40, 40, 40, 12]);

    let mut cube = Cube::default();
    let network_evaluation = network.evaluate(cube.to_ai_input());

    let mut network_decision: usize = 0;
    for (i, v) in network_evaluation.iter().enumerate() {
        if *v > network_evaluation[network_decision] {
            network_decision = i;
        }
    }

    println!("{:?}", network_evaluation);
    println!("Ai wants to do move nr: {}", network_decision);

    cube.apply_move_by_id(network_decision as i32);
    cube.display();
}
