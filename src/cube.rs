use colored::*;
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
