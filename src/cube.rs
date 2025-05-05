use colored::*;
use rand::Rng;
use std::fmt;

pub struct Cube {
    pub data: [[Color; 9]; 6],
}
impl Cube {
    #[allow(dead_code)]
    pub fn display(&self) {
        for y in 0..3 {
            for face in 0..6 {
                for x in 0..3 {
                    print!("{}", self.data[face][x + y * 3]);
                }
                print!(" ");
            }
            println!();
        }
    }

    #[allow(dead_code)]
    pub fn to_ai_input(&self) -> Vec<f32> {
        fn one_hot(index: usize, size: usize) -> Vec<f32> {
            (0..size).map(|i| if i == index { 1.0 } else { 0.0 }).collect()
        }

        let mut output = Vec::with_capacity(324);

        for face in self.data {
            for sticker in face {
                output.extend(match sticker {
                    Color::Yellow => one_hot(0, 6),
                    Color::White => one_hot(1, 6),
                    Color::Red => one_hot(2, 6),
                    Color::Orange => one_hot(3, 6),
                    Color::Green => one_hot(4, 6),
                    Color::Blue => one_hot(5, 6),
                    #[cfg(test)]
                    Color::Debug(_) => one_hot(0, 0)
                });
            }
        }

        output
    }

    #[allow(dead_code)]
    pub fn scramble(&mut self, moves_count: i32) {
        let mut rng = rand::rng();
        let mut last_move: i32 = rng.random_range(0..=11);

        //Avoiding the opposite variant of move, XOR swaps parity of last_move
        let mut avoided_move: i32 = last_move ^ 1;

        for _ in 0..moves_count {
            let mut new_move = rng.random_range(0..11);
            if new_move >= avoided_move { new_move += 1; }
            last_move = new_move;
            avoided_move = last_move ^ 1;

            self.apply_move_by_id(new_move);            
        }
    }

    #[allow(dead_code)]
    pub fn is_solved(&self) -> bool {
        self.data.iter().take(6).all(|face| {
            let face_color = face[4];
            face.iter().all(|&color| color == face_color)
        })
    }

    #[allow(dead_code)]
    pub fn apply_move_by_id(&mut self, move_id: i32){
        match move_id {
            0 => self.u(),
            1 => self.u_reverse(),
            2 => self.l(),
            3 => self.l_reverse(),
            4 => self.f(),
            5 => self.f_reverse(),
            6 => self.r(),
            7 => self.r_reverse(),
            8 => self.b(),
            9 => self.b_reverse(),
            10 => self.d(),
            11 => self.d_reverse(),
            _ => print!("Scrambling Went wrong")
        }
    }
}

impl Default for Cube {
    fn default() -> Self {
        Self {
            data: [
                [Color::Yellow; 9],
                [Color::Blue; 9], 
                [Color::Red; 9], 
                [Color::Green; 9], 
                [Color::Orange; 9], 
                [Color::White; 9]
            ]
        }
    }
}

#[derive(PartialEq, Eq, Default, Clone, Copy, Debug)]
pub enum Color {
    #[default]
    White,
    Yellow,
    Red,
    Orange,
    Green,
    Blue,
    #[cfg(test)]
    Debug(i32),
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self { 
            Color::White => write!(f, "{}", "  ".on_white()),
            Color::Yellow => write!(f, "{}", "  ".on_yellow()),
            Color::Red => write!(f, "{}", "  ".on_red()),
            Color::Orange => write!(f, "{}", "  ".on_truecolor(255, 150, 46)),
            Color::Green => write!(f, "{}", "  ".on_green()),
            Color::Blue => write!(f, "{}", "  ".on_blue()),
            #[cfg(test)]
            Color::Debug(i) => write!(f, "{:2}", i),
        }
    }
}