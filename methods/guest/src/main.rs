#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
// #![no_std]  // std support is experimental


// use risc0_zkvm::guest::env;

// risc0_zkvm::guest::entry!(main);


// fn main() {
//     // TODO: Implement your guest code here

//     // read the input
//     let input: u32 = env::read();

//     // TODO: do something with the input

//     // write public output to the journal
//     env::commit(&input);
// }

use risc0_zkvm::guest::env;
use smartcore::{
    linalg::basic::matrix::DenseMatrix,
    svm::{
        svc::{SVCParameters, SVC},
        Kernels,
    },
    // ensemble::random_forest_classifier::*,
    linear::linear_regression::LinearRegression,
};
// use std::println;
risc0_zkvm::guest::entry!(main);

fn main() {
    // Read in is_svm boolean to ensure the correct code block is executed
    let is_svm: bool = env::read();

    if !is_svm {
        // Read the model from the host into a SmartCore Decesion Tree model object.
        // We MUST explicitly declare the correct type in order for deserialization to be
        // successful.
        // type Model = RandomForestClassifier<f64, u8, DenseMatrix<f64>, Vec<u8>>;
        type Model = LinearRegression< f64, f64, DenseMatrix<f64>, Vec<f64>>;
        let trained_model: Model = env::read();

        // Read the input data into a DenseMatrix.
        let x_data: DenseMatrix<f64> = env::read();

        // We call the predict() function on our trained model to perform inference.
        let y_hat = trained_model.predict(&x_data).unwrap();

        // This line is optional and can be commented out, but it's useful to see
        // the output of the computation before the proving step begins.
        println!("answer: {:?}", &y_hat);

        // We commit the output to the journal.
        env::commit(&y_hat);
    } else {
        // Read the model from the host into a SmartCore SVC object.
        let mut model: SVC<f64, i32, DenseMatrix<f64>, Vec<i32>> = env::read();

        // Read the input data into a DenseMatrix.
        let x_data: DenseMatrix<f64> = env::read();

        // Calling predict on a deserialized SVM model will result in an error due to the missing parameters field.
        // We need to use THE EXACT SAME SVCParameters that we used to train the model.  Adjust the code below in accordance with how you trained the SVC model.
        let params_same = &SVCParameters::default()
            .with_c(200.0)
            .with_kernel(Kernels::linear());

        // Now we can update the model with params_same.  The RISC Zero fork changes the visibility of the parameters field of the SVC and SVR model structs to public to allow for this reinsertion
        model.parameters = Some(params_same);

        // Now that the parameters have been loaded back into the SVC model struct, we can call predict on the model.
        let y_hat: Vec<f64> = model.predict(&x_data).unwrap();

        // This line is optional and can be commented out, but it's useful to see
        // the output of the computation before the proving step begins.
        // println!("answer: {:?}", &y_hat);

        // We commit the output to the journal.
        env::commit(&y_hat);
    }
    // Logging the total cycle count is optional, though it's quite useful for benchmarking
    // the various operations in the guest code. env::cycle_count() can be
    // called anywhere in the guest, multiple times. So if we are interested in
    // knowing how many cycles the inference computation takes, we can calculate
    // total cycles before and after model.predict() and the difference between
    // the two values equals the total cycle count for that section of the guest
    // code.
    println!(
        "Total cycles for guest code execution: {}",
        env::cycle_count()
    );
}
