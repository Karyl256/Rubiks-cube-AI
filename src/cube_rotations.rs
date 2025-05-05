use crate::cube::*;

impl Cube {
    #[allow(dead_code)]
    pub fn r(&mut self) {
        // Rotate right side corners
        let temp = self.data[3][0];
        self.data[3][0] = self.data[3][6];
        self.data[3][6] = self.data[3][8];
        self.data[3][8] = self.data[3][2];
        self.data[3][2] = temp;
        // Rotate right side edges
        let temp = self.data[3][1];
        self.data[3][1] = self.data[3][3];
        self.data[3][3] = self.data[3][7];
        self.data[3][7] = self.data[3][5];
        self.data[3][5] = temp;
        // Rotate outside of right side
        let temp1 = self.data[0][8];
        let temp2 = self.data[0][5];
        let temp3 = self.data[0][2];
        
        self.data[0][8] = self.data[2][8];
        self.data[0][5] = self.data[2][5];
        self.data[0][2] = self.data[2][2];
        
        self.data[2][8] = self.data[5][8];
        self.data[2][5] = self.data[5][5];
        self.data[2][2] = self.data[5][2];
        
        self.data[5][8] = self.data[4][0];
        self.data[5][5] = self.data[4][3];
        self.data[5][2] = self.data[4][6];
        
        self.data[4][0] = temp1;
        self.data[4][3] = temp2;
        self.data[4][6] = temp3;
    }
    #[allow(dead_code)]
    pub fn r_reverse(&mut self) {
        // Reverse rotate right side corners
        let temp = self.data[3][0];
        self.data[3][0] = self.data[3][2];
        self.data[3][2] = self.data[3][8];
        self.data[3][8] = self.data[3][6];
        self.data[3][6] = temp;
        
        // Reverse rotate right side edges
        let temp = self.data[3][1];
        self.data[3][1] = self.data[3][5];
        self.data[3][5] = self.data[3][7];
        self.data[3][7] = self.data[3][3];
        self.data[3][3] = temp;
        
        // Reverse rotate outside of right side
        let temp1 = self.data[0][2];
        let temp2 = self.data[0][5];
        let temp3 = self.data[0][8];
        
        self.data[0][2] = self.data[4][6];
        self.data[0][5] = self.data[4][3];
        self.data[0][8] = self.data[4][0];
        
        self.data[4][6] = self.data[5][2];
        self.data[4][3] = self.data[5][5];
        self.data[4][0] = self.data[5][8];
        
        self.data[5][2] = self.data[2][2];
        self.data[5][5] = self.data[2][5];
        self.data[5][8] = self.data[2][8];
        
        self.data[2][2] = temp1;
        self.data[2][5] = temp2;
        self.data[2][8] = temp3;
    }
    #[allow(dead_code)]
    pub fn l(&mut self) {
        // Rotate left side corners
        let temp = self.data[1][0];
        self.data[1][0] = self.data[1][6];
        self.data[1][6] = self.data[1][8];
        self.data[1][8] = self.data[1][2];
        self.data[1][2] = temp;
        // Rotate left side edges
        let temp = self.data[1][1];
        self.data[1][1] = self.data[1][3];
        self.data[1][3] = self.data[1][7];
        self.data[1][7] = self.data[1][5];
        self.data[1][5] = temp;
        // Rotate outside of left side
        let temp1 = self.data[0][0];
        let temp2 = self.data[0][3];
        let temp3 = self.data[0][6];
        
        self.data[0][0] = self.data[4][8];
        self.data[0][3] = self.data[4][5];
        self.data[0][6] = self.data[4][2];
        
        self.data[4][8] = self.data[5][0];
        self.data[4][5] = self.data[5][3];
        self.data[4][2] = self.data[5][6];
        
        self.data[5][0] = self.data[2][0];
        self.data[5][3] = self.data[2][3];
        self.data[5][6] = self.data[2][6];
        
        self.data[2][0] = temp1;
        self.data[2][3] = temp2;
        self.data[2][6] = temp3;
    }
    #[allow(dead_code)]
    pub fn l_reverse(&mut self) {
        // Reverse rotate left side corners
        let temp = self.data[1][0];
        self.data[1][0] = self.data[1][2];
        self.data[1][2] = self.data[1][8];
        self.data[1][8] = self.data[1][6];
        self.data[1][6] = temp;

        // Reverse rotate left side edges
        let temp = self.data[1][1];
        self.data[1][1] = self.data[1][5];
        self.data[1][5] = self.data[1][7];
        self.data[1][7] = self.data[1][3];
        self.data[1][3] = temp;

        // Reverse rotate outside of left side
        let temp1 = self.data[0][0];
        let temp2 = self.data[0][3];
        let temp3 = self.data[0][6];

        self.data[0][0] = self.data[2][0];
        self.data[0][3] = self.data[2][3];
        self.data[0][6] = self.data[2][6];

        self.data[2][0] = self.data[5][0];
        self.data[2][3] = self.data[5][3];
        self.data[2][6] = self.data[5][6];

        self.data[5][0] = self.data[4][8];
        self.data[5][3] = self.data[4][5];
        self.data[5][6] = self.data[4][2];

        self.data[4][8] = temp1;
        self.data[4][5] = temp2;
        self.data[4][2] = temp3;
    }
    #[allow(dead_code)]
    pub fn u(&mut self) {
        // Rotate top corners
        let temp = self.data[0][0];
        self.data[0][0] = self.data[0][6];
        self.data[0][6] = self.data[0][8];
        self.data[0][8] = self.data[0][2];
        self.data[0][2] = temp;
        // Rotate top side edges
        let temp = self.data[0][1];
        self.data[0][1] = self.data[0][3];
        self.data[0][3] = self.data[0][7];
        self.data[0][7] = self.data[0][5];
        self.data[0][5] = temp;
        
        // Rotate outside of top side
        let temp1 = self.data[1][0];
        let temp2 = self.data[1][1];
        let temp3 = self.data[1][2];
        
        self.data[1][0] = self.data[2][0];
        self.data[1][1] = self.data[2][1];
        self.data[1][2] = self.data[2][2];
        
        self.data[2][0] = self.data[3][0];
        self.data[2][1] = self.data[3][1];
        self.data[2][2] = self.data[3][2];
        
        self.data[3][0] = self.data[4][0];
        self.data[3][1] = self.data[4][1];
        self.data[3][2] = self.data[4][2];
        
        self.data[4][0] = temp1;
        self.data[4][1] = temp2;
        self.data[4][2] = temp3;
    }
    #[allow(dead_code)]
    pub fn u_reverse(&mut self) {
        // Reverse rotate top corners
        let temp = self.data[0][0];
        self.data[0][0] = self.data[0][2];
        self.data[0][2] = self.data[0][8];
        self.data[0][8] = self.data[0][6];
        self.data[0][6] = temp;

        // Reverse rotate top side edges
        let temp = self.data[0][1];
        self.data[0][1] = self.data[0][5];
        self.data[0][5] = self.data[0][7];
        self.data[0][7] = self.data[0][3];
        self.data[0][3] = temp;

        // Reverse rotate outside of top side
        let temp1 = self.data[1][0];
        let temp2 = self.data[1][1];
        let temp3 = self.data[1][2];

        self.data[1][0] = self.data[4][0];
        self.data[1][1] = self.data[4][1];
        self.data[1][2] = self.data[4][2];

        self.data[4][0] = self.data[3][0];
        self.data[4][1] = self.data[3][1];
        self.data[4][2] = self.data[3][2];

        self.data[3][0] = self.data[2][0];
        self.data[3][1] = self.data[2][1];
        self.data[3][2] = self.data[2][2];

        self.data[2][0] = temp1;
        self.data[2][1] = temp2;
        self.data[2][2] = temp3;
    }
    #[allow(dead_code)]
    pub fn d(&mut self) {
        // Rotate bottom corners
        let temp = self.data[5][0];
        self.data[5][0] = self.data[5][6];
        self.data[5][6] = self.data[5][8];
        self.data[5][8] = self.data[5][2];
        self.data[5][2] = temp;
        // Rotate bottom side edges
        let temp = self.data[5][1];
        self.data[5][1] = self.data[5][3];
        self.data[5][3] = self.data[5][7];
        self.data[5][7] = self.data[5][5];
        self.data[5][5] = temp;

        // Rotate outside of bottom side
        let temp1 = self.data[1][6];
        let temp2 = self.data[1][7];
        let temp3 = self.data[1][8];

        self.data[1][6] = self.data[4][6];
        self.data[1][7] = self.data[4][7];
        self.data[1][8] = self.data[4][8];

        self.data[4][6] = self.data[3][6];
        self.data[4][7] = self.data[3][7];
        self.data[4][8] = self.data[3][8];

        self.data[3][6] = self.data[2][6];
        self.data[3][7] = self.data[2][7];
        self.data[3][8] = self.data[2][8];

        self.data[2][6] = temp1;
        self.data[2][7] = temp2;
        self.data[2][8] = temp3;
    }
    #[allow(dead_code)]
    pub fn d_reverse(&mut self) {
        // Reverse rotate bottom corners
        let temp = self.data[5][0];
        self.data[5][0] = self.data[5][2];
        self.data[5][2] = self.data[5][8];
        self.data[5][8] = self.data[5][6];
        self.data[5][6] = temp;
    
        // Reverse rotate bottom side edges
        let temp = self.data[5][1];
        self.data[5][1] = self.data[5][5];
        self.data[5][5] = self.data[5][7];
        self.data[5][7] = self.data[5][3];
        self.data[5][3] = temp;
    
        // Reverse rotate outside of bottom side
        let temp1 = self.data[1][6];
        let temp2 = self.data[1][7];
        let temp3 = self.data[1][8];
    
        self.data[1][6] = self.data[2][6];
        self.data[1][7] = self.data[2][7];
        self.data[1][8] = self.data[2][8];
    
        self.data[2][6] = self.data[3][6];
        self.data[2][7] = self.data[3][7];
        self.data[2][8] = self.data[3][8];
    
        self.data[3][6] = self.data[4][6];
        self.data[3][7] = self.data[4][7];
        self.data[3][8] = self.data[4][8];
    
        self.data[4][6] = temp1;
        self.data[4][7] = temp2;
        self.data[4][8] = temp3;
    }

    #[allow(dead_code)]
    pub fn f(&mut self) {
        //Rotate front corners
        let temp = self.data[2][0];
        self.data[2][0] = self.data[2][6];
        self.data[2][6] = self.data[2][8];
        self.data[2][8] = self.data[2][2];
        self.data[2][2] = temp;
        //Rotate front side edges
        let temp = self.data[2][1];
        self.data[2][1] = self.data[2][3];
        self.data[2][3] = self.data[2][7];
        self.data[2][7] = self.data[2][5];
        self.data[2][5] = temp;

        // Rotate outside of front side
        let temp1 = self.data[0][8];
        let temp2 = self.data[0][7];
        let temp3 = self.data[0][6];
        
        self.data[0][8] = self.data[1][2];
        self.data[0][7] = self.data[1][5];
        self.data[0][6] = self.data[1][8];
        
        self.data[1][2] = self.data[5][0];
        self.data[1][5] = self.data[5][1];
        self.data[1][8] = self.data[5][2];
        
        self.data[5][0] = self.data[3][6];
        self.data[5][1] = self.data[3][3];
        self.data[5][2] = self.data[3][0];
        
        self.data[3][6] = temp1;
        self.data[3][3] = temp2;
        self.data[3][0] = temp3;
    }

    #[allow(dead_code)]
    pub fn f_reverse(&mut self) {
        // Reverse rotate front corners
        let temp = self.data[2][2];
        self.data[2][2] = self.data[2][8];
        self.data[2][8] = self.data[2][6];
        self.data[2][6] = self.data[2][0];
        self.data[2][0] = temp;

        // Reverse rotate front side edges
        let temp = self.data[2][1];
        self.data[2][1] = self.data[2][5];
        self.data[2][5] = self.data[2][7];
        self.data[2][7] = self.data[2][3];
        self.data[2][3] = temp;

        // Reverse rotate outside of front side
        let temp1 = self.data[3][6];
        let temp2 = self.data[3][3];
        let temp3 = self.data[3][0];

        self.data[3][6] = self.data[5][0];
        self.data[3][3] = self.data[5][1];
        self.data[3][0] = self.data[5][2];

        self.data[5][0] = self.data[1][2];
        self.data[5][1] = self.data[1][5];
        self.data[5][2] = self.data[1][8];

        self.data[1][2] = self.data[0][8];
        self.data[1][5] = self.data[0][7];
        self.data[1][8] = self.data[0][6];

        self.data[0][8] = temp1;
        self.data[0][7] = temp2;
        self.data[0][6] = temp3;
    }

    #[allow(dead_code)]
    pub fn b(&mut self) {
        //Rotate front corners
        let temp = self.data[4][0];
        self.data[4][0] = self.data[4][6];
        self.data[4][6] = self.data[4][8];
        self.data[4][8] = self.data[4][2];
        self.data[4][2] = temp;
        //Rotate front side edges
        let temp = self.data[4][1];
        self.data[4][1] = self.data[4][3];
        self.data[4][3] = self.data[4][7];
        self.data[4][7] = self.data[4][5];
        self.data[4][5] = temp;

        // Rotate outside of front side
        let temp1 = self.data[0][0];
        let temp2 = self.data[0][1];
        let temp3 = self.data[0][2];
        
        self.data[0][0] = self.data[3][2];
        self.data[0][1] = self.data[3][5];
        self.data[0][2] = self.data[3][8];
        
        self.data[3][2] = self.data[5][8];
        self.data[3][5] = self.data[5][7];
        self.data[3][8] = self.data[5][6];
        
        self.data[5][8] = self.data[1][6];
        self.data[5][7] = self.data[1][3];
        self.data[5][6] = self.data[1][0];
        
        self.data[1][6] = temp1;
        self.data[1][3] = temp2;
        self.data[1][0] = temp3;
    }

    #[allow(dead_code)]
    pub fn b_reverse(&mut self) {
        // Reverse rotate back corners
        let temp = self.data[4][2];
        self.data[4][2] = self.data[4][8];
        self.data[4][8] = self.data[4][6];
        self.data[4][6] = self.data[4][0];
        self.data[4][0] = temp;

        // Reverse rotate back side edges
        let temp = self.data[4][1];
        self.data[4][1] = self.data[4][5];
        self.data[4][5] = self.data[4][7];
        self.data[4][7] = self.data[4][3];
        self.data[4][3] = temp;

        // Reverse rotate outside of back side
        let temp1 = self.data[1][6];
        let temp2 = self.data[1][3];
        let temp3 = self.data[1][0];

        self.data[1][6] = self.data[5][8];
        self.data[1][3] = self.data[5][7];
        self.data[1][0] = self.data[5][6];

        self.data[5][8] = self.data[3][2];
        self.data[5][7] = self.data[3][5];
        self.data[5][6] = self.data[3][8];

        self.data[3][2] = self.data[0][0];
        self.data[3][5] = self.data[0][1];
        self.data[3][8] = self.data[0][2];

        self.data[0][0] = temp1;
        self.data[0][1] = temp2;
        self.data[0][2] = temp3;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn debug_cube() -> Cube {
        let mut cube = Cube::default();
        for f in 0..6 {
            for i in 0..9 {
                cube.data[f][i] = Color::Debug((f * 9 + i) as i32);
            }
        }
        cube
    }

    fn check_if_correct(left: &Cube, right: &Cube) -> Result<(), Vec<(Color, Color, usize)>> {
        // Error (expected color, recieved color, position)
        let mut errors = Vec::new();
        for face in 0..6 {
            for sticker in 0..9 {
                if left.data[face][sticker] != right.data[face][sticker] {
                    errors.push((left.data[face][sticker], right.data[face][sticker], face * 9 + sticker));
                }
            }
        }
        
        if errors.is_empty() {
            return Ok(());
        }
        else {
            return Err(errors);
        }
    }

    #[test]
    fn test_checking() {
        let cube_a = Cube::default();
        let mut cube_b = Cube::default();
        cube_b.data[0][0] = Color::Debug(1);

        //Check if function returns properly data from comparing cube_a and cube_b
        assert_eq!(Err(vec![(cube_a.data[0][0], cube_b.data[0][0], 0)]), check_if_correct(&cube_a, &cube_b));
        //Check if it is ok if it gets 2 same cubes
        assert_eq!(Ok(()), check_if_correct(&debug_cube(), &debug_cube()));
    }

    fn rotate_face_right(face: &mut [Color; 9]) {
        // Rotate corners
        let temp = face[0];
        face[0] = face[6];
        face[6] = face[8];
        face[8] = face[2];
        face[2] = temp;

        // Rotate edges
        let temp = face[1];
        face[1] = face[3];
        face[3] = face[7];
        face[7] = face[5];
        face[5] = temp;
    }

    #[test]
    fn test_up_face_rotation() {
        let initial_state = debug_cube();
        let mut expected_after_u = debug_cube();
        expected_after_u.data[1][0] = initial_state.data[2][0];
        expected_after_u.data[1][1] = initial_state.data[2][1];
        expected_after_u.data[1][2] = initial_state.data[2][2];

        expected_after_u.data[2][0] = initial_state.data[3][0];
        expected_after_u.data[2][1] = initial_state.data[3][1];
        expected_after_u.data[2][2] = initial_state.data[3][2];
        
        expected_after_u.data[3][0] = initial_state.data[4][0];
        expected_after_u.data[3][1] = initial_state.data[4][1];
        expected_after_u.data[3][2] = initial_state.data[4][2];
        
        expected_after_u.data[4][0] = initial_state.data[1][0];
        expected_after_u.data[4][1] = initial_state.data[1][1];
        expected_after_u.data[4][2] = initial_state.data[1][2];

        rotate_face_right(&mut expected_after_u.data[0]);

        let mut actual_after_u = debug_cube();
        actual_after_u.u();
        assert_eq!(Ok(()), check_if_correct(&expected_after_u, &actual_after_u));

        let mut reversibility_test_cube = debug_cube();
        reversibility_test_cube.u();
        reversibility_test_cube.u_reverse();
        assert_eq!(Ok(()), check_if_correct(&debug_cube(), &reversibility_test_cube));
    }

    #[test]
    fn test_left_face_rotation() {
        let initial_state = debug_cube();
        let mut expected_after_l = debug_cube();
        expected_after_l.data[0][0] = initial_state.data[4][8];
        expected_after_l.data[0][3] = initial_state.data[4][5];
        expected_after_l.data[0][6] = initial_state.data[4][2];

        expected_after_l.data[4][8] = initial_state.data[5][0];
        expected_after_l.data[4][5] = initial_state.data[5][3];
        expected_after_l.data[4][2] = initial_state.data[5][6];
        
        expected_after_l.data[5][0] = initial_state.data[2][0];
        expected_after_l.data[5][3] = initial_state.data[2][3];
        expected_after_l.data[5][6] = initial_state.data[2][6];
        
        expected_after_l.data[2][0] = initial_state.data[0][0];
        expected_after_l.data[2][3] = initial_state.data[0][3];
        expected_after_l.data[2][6] = initial_state.data[0][6];

        rotate_face_right(&mut expected_after_l.data[1]);

        let mut actual_after_l = debug_cube();
        actual_after_l.l();
        assert_eq!(Ok(()), check_if_correct(&expected_after_l, &actual_after_l));

        let mut reversibility_test_cube = debug_cube();
        reversibility_test_cube.l();
        reversibility_test_cube.l_reverse();
        assert_eq!(Ok(()), check_if_correct(&debug_cube(), &reversibility_test_cube));
    }

    #[test]
    fn test_front_face_rotation() {
        let initial_state = debug_cube();
        let mut expected_after_f = debug_cube();
        expected_after_f.data[0][6] = initial_state.data[1][8];
        expected_after_f.data[0][7] = initial_state.data[1][5];
        expected_after_f.data[0][8] = initial_state.data[1][2];

        expected_after_f.data[1][8] = initial_state.data[5][2];
        expected_after_f.data[1][5] = initial_state.data[5][1];
        expected_after_f.data[1][2] = initial_state.data[5][0];
        
        expected_after_f.data[5][2] = initial_state.data[3][0];
        expected_after_f.data[5][1] = initial_state.data[3][3];
        expected_after_f.data[5][0] = initial_state.data[3][6];
        
        expected_after_f.data[3][0] = initial_state.data[0][6];
        expected_after_f.data[3][3] = initial_state.data[0][7];
        expected_after_f.data[3][6] = initial_state.data[0][8];

        rotate_face_right(&mut expected_after_f.data[2]);

        let mut actual_after_f = debug_cube();
        actual_after_f.f();
        assert_eq!(Ok(()), check_if_correct(&expected_after_f, &actual_after_f));

        let mut reversibility_test_cube = debug_cube();
        reversibility_test_cube.f();
        reversibility_test_cube.f_reverse();
        assert_eq!(Ok(()), check_if_correct(&debug_cube(), &reversibility_test_cube));
    }

    #[test]
    fn test_right_face_rotation() {
        let initial_state = debug_cube();
        let mut expected_after_r = debug_cube();
        expected_after_r.data[0][2] = initial_state.data[2][2];
        expected_after_r.data[0][5] = initial_state.data[2][5];
        expected_after_r.data[0][8] = initial_state.data[2][8];

        expected_after_r.data[2][2] = initial_state.data[5][2];
        expected_after_r.data[2][5] = initial_state.data[5][5];
        expected_after_r.data[2][8] = initial_state.data[5][8];
        
        expected_after_r.data[5][2] = initial_state.data[4][6];
        expected_after_r.data[5][5] = initial_state.data[4][3];
        expected_after_r.data[5][8] = initial_state.data[4][0];
        
        expected_after_r.data[4][6] = initial_state.data[0][2];
        expected_after_r.data[4][3] = initial_state.data[0][5];
        expected_after_r.data[4][0] = initial_state.data[0][8];

        rotate_face_right(&mut expected_after_r.data[3]);

        let mut actual_after_r = debug_cube();
        actual_after_r.r();
        assert_eq!(Ok(()), check_if_correct(&expected_after_r, &actual_after_r));

        let mut reversibility_test_cube = debug_cube();
        reversibility_test_cube.r();
        reversibility_test_cube.r_reverse();
        assert_eq!(Ok(()), check_if_correct(&debug_cube(), &reversibility_test_cube));
    }

    #[test]
    fn test_back_face_rotation() {
        let initial_state = debug_cube();
        let mut expected_after_b = debug_cube();
        expected_after_b.data[0][2] = initial_state.data[3][8];
        expected_after_b.data[0][1] = initial_state.data[3][5];
        expected_after_b.data[0][0] = initial_state.data[3][2];

        expected_after_b.data[3][8] = initial_state.data[5][6];
        expected_after_b.data[3][5] = initial_state.data[5][7];
        expected_after_b.data[3][2] = initial_state.data[5][8];
        
        expected_after_b.data[5][6] = initial_state.data[1][0];
        expected_after_b.data[5][7] = initial_state.data[1][3];
        expected_after_b.data[5][8] = initial_state.data[1][6];
        
        expected_after_b.data[1][0] = initial_state.data[0][2];
        expected_after_b.data[1][3] = initial_state.data[0][1];
        expected_after_b.data[1][6] = initial_state.data[0][0];

        rotate_face_right(&mut expected_after_b.data[4]);

        let mut actual_after_b = debug_cube();
        actual_after_b.b();
        assert_eq!(Ok(()), check_if_correct(&expected_after_b, &actual_after_b));

        let mut reversibility_test_cube = debug_cube();
        reversibility_test_cube.b();
        reversibility_test_cube.b_reverse();
        assert_eq!(Ok(()), check_if_correct(&debug_cube(), &reversibility_test_cube));
    }

    #[test]
    fn test_down_face_rotation() {
        let initial_state = debug_cube();
        let mut expected_after_down = debug_cube();
        expected_after_down.data[1][8] = initial_state.data[4][8];
        expected_after_down.data[1][7] = initial_state.data[4][7];
        expected_after_down.data[1][6] = initial_state.data[4][6];

        expected_after_down.data[4][8] = initial_state.data[3][8];
        expected_after_down.data[4][7] = initial_state.data[3][7];
        expected_after_down.data[4][6] = initial_state.data[3][6];
        
        expected_after_down.data[3][8] = initial_state.data[2][8];
        expected_after_down.data[3][7] = initial_state.data[2][7];
        expected_after_down.data[3][6] = initial_state.data[2][6];
        
        expected_after_down.data[2][8] = initial_state.data[1][8];
        expected_after_down.data[2][7] = initial_state.data[1][7];
        expected_after_down.data[2][6] = initial_state.data[1][6];

        rotate_face_right(&mut expected_after_down.data[5]);

        let mut actual_after_d = debug_cube();
        actual_after_d.d();
        assert_eq!(Ok(()), check_if_correct(&expected_after_down, &actual_after_d));

        let mut reversibility_test_cube = debug_cube();
        reversibility_test_cube.d();
        reversibility_test_cube.d_reverse();
        assert_eq!(Ok(()), check_if_correct(&debug_cube(), &reversibility_test_cube));
    }
    
}