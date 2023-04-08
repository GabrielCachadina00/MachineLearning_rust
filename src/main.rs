//use std::io;
pub mod act_fnc; //Stores all posible activation functions
pub mod matrix_ops; //Stores all matrix operations
pub mod neuron_ops; //Operates with the neuron layers


use crate::matrix_ops::Matrix;

fn main() {
    let input = Matrix{
        rows: 2,
        cols: 1,
        values: vec![vec![0.0],vec![1.0]],
    };
    matrix_ops::matrix_show(&input);



    // Creation of the layers
    let mut layer1 = neuron_ops::layer_creation(8,4,"relu");//layer neurons,next layer neurons,activation
    let mut layer2 = neuron_ops::layer_creation(2,3,"sigmoid");//layer neurons,next layer neurons,activation

    

    //Initialization of the layers
    layer1 = neuron_ops::bias_init_runiform(layer1,0.0,1.0); //Initialize the bias between the min and max of the selected layer
    layer2 = neuron_ops::bias_init_0s(layer2); //Initialize to 0s

    matrix_ops::matrix_show(&layer2.biases);
    matrix_ops::matrix_show(&layer1.biases);

}