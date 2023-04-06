//Matrix operations
use rand::Rng; //Random numbers

//Init a matrix rows x columns with 0s
pub fn matrix_init_0 (row:u32,col:u32)->Vec<Vec<f64>>{
    let matrix = vec![vec![0.32;row.try_into().unwrap()];col.try_into().unwrap()]; //conversion because vec needs usize and row/col are u32
    return matrix;
}


// NOT YET WORKING
//Init a matrix rows x columns with a normal distribution from min to max
pub fn matrix_init_runiform (row:u32,col:u32,min:i32,max:i32)->Vec<Vec<f64>>{
    let mut matrix = matrix_init_0(row,col);
    
    let mut rng = rand::thread_rng();

    for (i, row) in matrix.iter().enumerate(){
        for (j,value) in row.iter().enumerate(){
            let r = rng.gen::<f64>();
            let sumation = (min.abs() + max.abs()) as f64;
            println!("{i},{j}");

            //matrix[i][j] = (r*sumation) - (min.abs()) as f64; //Should do the operation and save the value in [i][j], but I have already borrowed it, looking for a solution

        }
    }

    return matrix;
}



// Show a matrix
pub fn matrix_show (matrix:Vec<Vec<f64>>){

    for (i, row) in matrix.iter().enumerate(){
        print!("|");
        for (j,value) in row.iter().enumerate(){
            print!(" {} ",value)
        }
        print!("|\n");
    }
}
