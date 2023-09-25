use multiply_methods::MULTIPLY_ELF; // It is a binary file of multiply_method
use risc0_zkvm::{
    default_prover,
    serde::{from_slice, to_vec},
    ExecutorEnv,
  };

fn main() {

    // Declaring our secret input params
    let a: u64 = 17;
    let b: u64 = 23;

    // First, we construct an executor environment
    let env = ExecutorEnv::builder()
    .add_input(&to_vec(&a).unwrap())  // Passing the input params to environment so it can be used by gues proggram
    .add_input(&to_vec(&b).unwrap())
    .build()
    .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, MULTIPLY_ELF).unwrap();

    // Extract journal of receipt (i.e. output c, where c = a * b)
    let c: u64 = from_slice(&receipt.journal).unwrap();

    // Print an assertion
    println!("Hello, world! I know the factors of {}, and I can prove it!", c);

    // Let's serialize the receipt so we can save it to an file for verifier program to verify.
    let serialized = bincode::serialize(&receipt).unwrap();

    // Writing the serialized contect to receipt.bin file
    let _saved_file = match std::fs::write("./receipt.bin", serialized){
     Ok(()) => println!("Receipt saved and serialized as receipt.bin"),
     Err(_) => println!("Something went wrong !!"),
    };
}
