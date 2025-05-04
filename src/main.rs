mod cube;
mod cube_rotations;

use cube::Cube;

fn main() {
    let mut cube = Cube::default();
    cube.print(); println!();
    
    cube.f();
    cube.print(); println!();
    cube.l();
    cube.print(); println!();
    cube.f();
    cube.print(); println!();
    cube.u_reverse();
    cube.print(); println!();
    cube.r();
    cube.print(); println!();
    cube.u();
    cube.print(); println!();
    cube.f();
    cube.print(); println!();
    cube.f();
    cube.print(); println!();
    cube.l();
    cube.print(); println!();
    cube.l();
    cube.print(); println!();
    cube.u_reverse();
    cube.print(); println!();
    cube.l_reverse();
    cube.print(); println!();
    cube.b();
    cube.print(); println!();
    cube.d_reverse();
    cube.print(); println!();
    cube.b(); cube.b(); cube.b();
    cube.print(); println!();
    cube.l();
    cube.print(); println!();
    cube.l();
    cube.print(); println!();
    cube.u();
    cube.print(); println!();
    //Up wrong! (mirrored)
    //Back bottom left kinda wrong propably because of other algorithms
    //Also at back top is flipped
}
