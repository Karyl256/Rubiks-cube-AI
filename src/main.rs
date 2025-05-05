mod cube;
mod cube_rotations;

use cube::Cube;

fn main() {
    let mut cube = Cube::default();
    cube.print();
    cube.scramble(3);
    cube.print();
}
