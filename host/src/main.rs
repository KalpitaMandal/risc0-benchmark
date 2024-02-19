// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use methods::{
    KALPITA_ELF, KALPITA_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv};
use smartcore::{
    linalg::basic::matrix::DenseMatrix, 
    // ensemble::random_forest_classifier::*,
    linear::linear_regression::LinearRegression,
};
use std::time::Instant;

const MODEL_SERIALIZED: &[u8] = include_bytes!("../../res/ml-model/random_forest_model_bytes.bin");
const DATA_SERIALIZED: &[u8] = include_bytes!("../../res/input-data/random_forest_data_bytes.bin");

fn main() {
    let result = predict();
    println!("Prediction recorded in journal is: {:?}", &result);
}

fn predict() -> Vec<u32> {
    // We set a boolean to establish whether we are using a SVM model.  This will be
    // passed to the guest and is important for execution of the guest code.
    // SVM models require an extra step that is not required of other SmartCore
    // models.
    let is_svm: bool = false;

    let model_fit_ts = Instant::now();
    // Deserialize the data from rmp into native rust types.
    // type Model = RandomForestClassifier<f64, u8, DenseMatrix<f64>, Vec<u8>>;
    type Model = LinearRegression< f64, f64, DenseMatrix<f64>, Vec<f64>>;
    let model: Model =
        rmp_serde::from_slice(&MODEL_SERIALIZED).expect("model failed to deserialize byte array");
    let data: DenseMatrix<f64> =
        rmp_serde::from_slice(&DATA_SERIALIZED).expect("data failed to deserialize byte array");

    let env = ExecutorEnv::builder()
        .write(&is_svm)
        .expect("bool failed to serialize")
        .write(&model)
        .expect("model failed to serialize")
        .write(&data)
        .expect("data failed to serialize")
        .build()
        .unwrap();

    println!("Model fitting and setup time: {:?}", model_fit_ts.elapsed());

    // Obtain the default prover.
    // Note that for development purposes we do not need to run the prover. To
    // bypass the prover, use:
    // ```
    // RISC0_DEV_MODE=1 cargo run -r
    // ```
    let prove_ts = Instant::now();
    println!("Starting proof setup");
    let prover = default_prover();

    // This initiates a session, runs the STARK prover on the resulting exection
    // trace, and produces a receipt.
    println!("Start proving");
    let receipt = prover.prove(env, KALPITA_ELF).unwrap();
    println!("Proving time: {:?}", prove_ts.elapsed());

    // We read the result that the guest code committed to the journal. The
    // receipt can also be serialized and sent to a verifier.
    let output: Vec<u32> = receipt.journal.decode().unwrap();

    // The receipt was verified at the end of proving, but the below code is an
    // example of how someone else could verify this receipt.
    let verify_ts = Instant::now();
    receipt
        .verify(KALPITA_ID)
        .unwrap();
    println!("Verifying time: {:?}", verify_ts.elapsed());
    println!("Generated a proof of guest execution! {:?} is a public output from journal", output);
    output
}

// fn main() {
//     // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
//     tracing_subscriber::fmt()
//         .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
//         .init();

//     // An executor environment describes the configurations for the zkVM
//     // including program inputs.
//     // An default ExecutorEnv can be created like so:
//     // `let env = ExecutorEnv::builder().build().unwrap();`
//     // However, this `env` does not have any inputs.
//     //
//     // To add add guest input to the executor environment, use
//     // ExecutorEnvBuilder::write().
//     // To access this method, you'll need to use ExecutorEnv::builder(), which
//     // creates an ExecutorEnvBuilder. When you're done adding input, call
//     // ExecutorEnvBuilder::build().

//     // For example:
//     let input: u32 = 15 * u32::pow(2, 27) + 1;
//     let env = ExecutorEnv::builder()
//         .write(&input)
//         .unwrap()
//         .build()
//         .unwrap();

//     // Obtain the default prover.
//     let prover = default_prover();

//     // Produce a receipt by proving the specified ELF binary.
//     let receipt = prover
//         .prove(env, KALPITA_ELF)
//         .unwrap();

//     // TODO: Implement code for retrieving receipt journal here.

//     // For example:
//     let _output: u32 = receipt.journal.decode().unwrap();

//     // The receipt was verified at the end of proving, but the below code is an
//     // example of how someone else could verify this receipt.
//     receipt
//         .verify(KALPITA_ID)
//         .unwrap();
// }