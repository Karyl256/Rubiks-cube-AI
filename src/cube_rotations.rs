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
        
        self.data[0][8] = self.data[2][2];
        self.data[0][5] = self.data[2][5];
        self.data[0][2] = self.data[2][8];
        
        self.data[2][2] = self.data[5][2];
        self.data[2][5] = self.data[5][5];
        self.data[2][8] = self.data[5][8];
        
        self.data[5][2] = self.data[4][6];
        self.data[5][5] = self.data[4][3];
        self.data[5][8] = self.data[4][0];
        
        self.data[4][6] = temp1;
        self.data[4][3] = temp2;
        self.data[4][0] = temp3;
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
        let temp1 = self.data[0][8];
        let temp2 = self.data[0][5];
        let temp3 = self.data[0][2];
        
        self.data[0][8] = self.data[4][6];
        self.data[0][5] = self.data[4][3];
        self.data[0][2] = self.data[4][0];
        
        self.data[4][6] = self.data[5][2];
        self.data[4][3] = self.data[5][5];
        self.data[4][0] = self.data[5][8];
        
        self.data[5][2] = self.data[2][2];
        self.data[5][5] = self.data[2][5];
        self.data[5][8] = self.data[2][8];
        
        self.data[2][2] = temp3;
        self.data[2][5] = temp2;
        self.data[2][8] = temp1;
    }
    #[allow(dead_code)]
    pub fn l(&mut self) {
        // Rotate left side corners
        let temp = self.data[1][0];
        self.data[1][6] = self.data[1][8];
        self.data[1][0] = self.data[1][6];
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
        
        self.data[0][0] = self.data[4][2];
        self.data[0][3] = self.data[4][5];
        self.data[0][6] = self.data[4][8];
        
        self.data[4][2] = self.data[5][0];
        self.data[4][5] = self.data[5][3];
        self.data[4][8] = self.data[5][6];
        
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

        self.data[5][0] = self.data[4][2];
        self.data[5][3] = self.data[4][5];
        self.data[5][6] = self.data[4][8];

        self.data[4][2] = temp3;
        self.data[4][5] = temp2;
        self.data[4][8] = temp1;
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
        
        self.data[4][0] = temp3;
        self.data[4][1] = temp2;
        self.data[4][2] = temp1;
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


}