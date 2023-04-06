//use std::io;
pub mod act_fnc; //Stores all posible activation functions
pub mod matrix_ops; //Stores all matrix operations


fn main() {
    //let a = act_fnc::act_relu(-23.3);
    //let a = act_fnc::act_lrelu(-23.3,3.14159/4.0);
    //let a = act_fnc::act_sigmoid(100.0);


    //let mut matrix = matrix_ops::matrix_init_0(2,3);

    

    let matrix = matrix_ops::matrix_init_runiform(2,3,-1,1);


    matrix_ops::matrix_show(matrix);
}