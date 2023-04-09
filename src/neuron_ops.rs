//Operations related with neurons

use crate::Matrix;
use crate::matrix_ops::matrix_init_0;
use crate::matrix_ops::matrix_init_runiform;
use crate::matrix_ops::matrix_mul;
use crate::matrix_ops::matrix_sum;
use crate::matrix_ops::matrix_show;



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
        neurons:matrix_init_0(a0,1),
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



// Weight initialization
pub fn weight_init_runiform(mut result:Layer,min:f64,max:f64)->Layer{
    result.weights = matrix_init_runiform(result.weights.rows,result.weights.cols,min,max);
    return result;
}

pub fn weight_init_0s(mut result:Layer)->Layer{
    result.weights = matrix_init_0(result.weights.rows,result.weights.cols);
    return result;
}



//------------------------------------------------------------------------------------------
//                          Feed forward
//------------------------------------------------------------------------------------------

pub fn model_ff(model:&mut [Layer],input:&Matrix){

    for i in (0..model.len()-1){
        println!("{}",model[i].activation);

        if i == 0{//Using input matrix
            println!("Changing input to layer");
            model[0].neurons.values = (*input.values).to_vec();
        }

        

        model[i+1].neurons = matrix_mul(&model[i].weights,&model[i].neurons);

        model[i+1].neurons = matrix_sum(&model[i+1].neurons,&model[i].biases);

        //Needs the activation fnc

    }

    //Last layer is the same but outputs to a new matrix

}