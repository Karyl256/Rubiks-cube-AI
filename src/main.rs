mod cube;
mod cube_rotations;
mod neural_network;

use neural_network::NeuralNetwork;
use cube::Cube;
use rand::seq::IndexedRandom;
use rand::Rng;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

fn main() {
    //Limit threads count
    let desired_threads = 5;
    ThreadPoolBuilder::new()
        .num_threads(desired_threads)
        .build_global()
        .unwrap();


    // Active neural networks
    let mut pool = Vec::new();
    let pool_count = 30;

    pool.push(NeuralNetwork::load_network("sigma.bin"));
    for _ in 0..pool_count {
        //New network with 324, 200, 100, 50, 25, 12 managed to get 18.33 avg after 2000 generations
        pool.push(NeuralNetwork::new(vec![324, 40, 40, 40, 40, 12]));
    }

    for generation in 0..2000 {
        // NN tries solving cube, networks_score stores how many times each network solved the cube
        let networks_score: Vec<usize> = pool.par_iter().map(|network| {
            let mut score = 0;
            for _ in 0..60 {
                let mut cube = Cube::default();
                cube.scramble(2);
                if network.solve(&mut cube, 2) {
                    score += 1;
                }
            }
            score
        }).collect();

        let mut scored: Vec<(usize,&NeuralNetwork)> = pool.iter().enumerate().collect();
        scored.sort_by_key(|&(i, _)| -(networks_score[i] as isize));

        // Keep only top 8
        let elites_num = 8;
        let elites: Vec<NeuralNetwork> = scored.iter().take(elites_num).map(|&(_, n)| n.clone()).collect();

        let avg_score: f64 = networks_score.iter().sum::<usize>() as f64 / networks_score.len() as f64;
        println!("Generation: {}, Top score: {}, Avg score: {:.4}, Min score: {}", generation, networks_score[scored[0].0], avg_score, networks_score[scored.last().unwrap().0]);

        // Rebuild pool
        pool.clear();
        pool.extend(elites.iter().cloned());

        while pool.len() < pool_count {
            if rand::rng().random_bool(0.1) { // 10% chance of new network
                pool.push(NeuralNetwork::new(vec![324, 40, 40, 40, 40, 12]));
            }
            else {
                let parent = elites.choose(&mut rand::rng()).unwrap();
                pool.push(parent.mutate(0.05, 0.2));
            }
        }
    }
    //pool.get(0).unwrap().save("sigma.bin");
}

impl NeuralNetwork {
    pub fn solve(&self, cube: &mut Cube, max_moves: usize) -> bool {
        for _ in 0..max_moves {
            //Evaluate
            let network_evaluation = self.evaluate(cube.to_ai_input());

            let mut network_decision: usize = 0;
            for (i, v) in network_evaluation.iter().enumerate() {
                if *v > network_evaluation[network_decision] {
                    network_decision = i;
                }
            }

            // Move cube like AI said
            cube.apply_move_by_id(network_decision as i32);

            if cube.is_solved() {
                return true;
            }
        }
        false
    }
}
