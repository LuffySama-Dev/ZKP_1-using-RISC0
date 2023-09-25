use hello_world_methods::MULTIPLY_ELF;
use risc0_zkvm::{
    default_prover,
    serde::{from_slice, to_vec},
    ExecutorEnv,
  };

fn main() {

    let a: u64 = 17;
    let b: u64 = 23;

    // First, we construct an executor environment
    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&a).unwrap())
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

    let serialized = bincode::serialize(&receipt).unwrap();

    let _saved_file = match std::fs::write("./receipt.bin", serialized){
         Ok(()) => println!("Receipt saved and serialized as receipt.bin"),
         Err(_) => println!("Something went wrong !!"),
    };

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    // receipt.verify(MULTIPLY_ID).unwrap();

}
