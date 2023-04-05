//Where X is the input to the ReLU function
pub fn act_relu (x:f64)->f64{
    if x < 0.0{
        return 0.0;
    }
    else{
        return x;
    }
}

//Where X is the input to the LReLU and alpha is the inclination
//angle of the Leaky ReLU
pub fn act_lrelu (x:f64,alpha:f64)->f64{ 
    if x < 0.0{
        return alpha.tan() * x;
    }
    else{
        return x;
    }
}


//Where X is the input to the Sigmoid
pub fn act_sigmoid (x:f64)->f64{ 
    let e:f64 = 2.71828;
 
    return 1.0/(1.0+ e.powf(-x));
}