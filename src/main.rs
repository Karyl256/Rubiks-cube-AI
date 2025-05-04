mod cube;
mod cube_rotations;

use cube::Cube;

fn main() {
    let mut cube = Cube::default();
    cube.print();
    println!();
    cube.f();
    cube.d();
    cube.r();
    cube.l();
    cube.u();
    cube.print();
    //Up wrong! (mirrored)
    //Back bottom left kinda wrong propably because of other algorithms
    //Also at back top is flipped
}
