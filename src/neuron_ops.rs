//Operations related with neurons

use crate::Matrix;
use crate::matrix_ops::matrix_init_0;
use crate::matrix_ops::matrix_init_runiform;
//use crate::matrix_ops::matrix_mul;


pub struct Layer{
    pub weights:Matrix,
    pub biases:Matrix,
    pub neurons:Matrix,
    pub activation:String,
}


//------------------------------------------------------------------------------------------
//               Creation of the layers of the neural network
//------------------------------------------------------------------------------------------

pub fn layer_creation(a0:usize,a1:usize,activation:&str)->Layer{
    let result = Layer{
        weights:matrix_init_0(a1,a0),
        biases:matrix_init_0(a1,1),
        neurons:matrix_init_0(a1,1),
        activation:activation.to_string(),
    };   
    return result;
}


//------------------------------------------------------------------------------------------
//                          Initialization
//------------------------------------------------------------------------------------------

// Bias initializations
pub fn bias_init_runiform(mut result:Layer,min:f64,max:f64)->Layer{
    result.biases = matrix_init_runiform(result.biases.rows,result.biases.cols,min,max);
    return result;
}

pub fn bias_init_0s(mut result:Layer)->Layer{
    result.biases = matrix_init_0(result.biases.rows,result.biases.cols);
    return result;
}