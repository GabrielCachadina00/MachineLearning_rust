//  MATRIX OPERATIONS



use rand::Rng; //Random numbers


pub struct Matrix{
    pub rows:usize,
    pub cols:usize,
    pub values:Vec<Vec<f64>>
}

//------------------------------------------------------------------------------------------
//                          Initialization methods
//------------------------------------------------------------------------------------------
//Init a matrix rows x columns with 0s
pub fn matrix_init_0 (nrow:usize,ncol:usize)->Matrix{

    let matrix = Matrix{
        rows: nrow,
        cols: ncol,
        values: vec![vec![0.0;ncol];nrow],
    };

    return matrix;
}

//Init a matrix rows x columns with a uniform distribution from min to max
pub fn matrix_init_runiform(nrow:usize,ncol:usize,min:f64,max:f64)->Matrix{
    let mut rng = rand::thread_rng(); //initialize rng thread   
    let mut matrix = matrix_init_0(nrow,ncol); //Initialize the matrix to 0s

    for i in 0..matrix.rows{
        for j in 0..matrix.cols{
            let r = rng.gen::<f64>();                           //Float between 0 and 1
            let sumation = min.abs() + max.abs();

            matrix.values[i][j] = (r*sumation) - min.abs();    //value = (random*(absmin+absmax))-absmin
        }                                                       
    }

    return matrix;
}

//------------------------------------------------------------------------------------------
//                          Matrix operations
//------------------------------------------------------------------------------------------

//Matrix multiplication
pub fn matrix_mul (a:&Matrix,b:&Matrix)->Matrix{
    //Can it be done?
    if (a.rows != b.cols) || (a.cols != b.rows){
        eprintln!("Not valid dimensions for matrix multiplication!");
        std::process::exit(1)
    }


    let mut result = matrix_init_0(a.rows,b.cols); //Initialize the matrix to 0s

    for i in 0..a.rows{
        for j in 0..b.cols{
            for k in 0..a.cols{
                result.values[i][j] = a.values[i][k]*b.values[k][j] + result.values[i][j];
            }
        }
    }
    return result;
}

//Matrix sum
pub fn matrix_sum (a:&Matrix,b:&Matrix)->Matrix{
    //Can it be done?
    if (a.rows != b.rows) || (a.cols != b.cols){
        eprintln!("Not valid dimensions for matrix sum!");
        std::process::exit(1)
    }


    let mut result = matrix_init_0(a.rows,a.cols); //Initialize the matrix to 0s

    for i in 0..a.rows{
        for j in 0..a.cols{
            result.values[i][j] = a.values[i][j] + b.values[i][j];
        }
    }

    return result;
}




//------------------------------------------------------------------------------------------
//                          Visualization of matrixes
//------------------------------------------------------------------------------------------

// Show a matrix
pub fn matrix_show (matrix:&Matrix){
    for i in 0..matrix.rows{
        print!("|");
        for j in 0..matrix.cols{
            print!("   {}   ",matrix.values[i][j]);
        }
        print!("|\n");
    }
    println!("\n\n\n");
}
