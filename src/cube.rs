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



#[allow(dead_code)]
#[derive(Default)]
#[derive(Clone)]
#[derive(Copy)]
pub enum Color {
    #[default]
    White,
    Yellow,
    Red,
    Orange,
    Green,
    Blue
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
        }
    }
}

impl Default for Cube {
    fn default() -> Self {
        let mut data: [[Color; 9]; 6] = [[const { Color::White }; 9]; 6];
        let faces_colors = [Color::Yellow, Color::Blue, Color::Red, Color::Green, Color::Orange, Color::White];
        for (face_index, color) in faces_colors.iter().enumerate() {
            for i in 0..9 {
                data[face_index][i] =  *color;
            }
        }
        Self {
            data
        }
    }
}
