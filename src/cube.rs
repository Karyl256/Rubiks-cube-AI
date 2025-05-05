use colored::*;
use rand::Rng;
use std::fmt;

pub struct Cube {
    pub data: [[Color; 9]; 6],
}
impl Cube {
    pub fn print(&self) {
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
}

impl Cube {
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

            match new_move {
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

    #[allow(dead_code)]
    pub fn check_if_solved(&self) -> bool {
        self.data.iter().take(6).all(|face| {
            let face_color = face[4];
            face.iter().all(|&color| color == face_color)
        })
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