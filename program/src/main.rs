// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    let hash = sp1_zkvm::io::read::<[u8; 32]>();
    let preimage = sp1_zkvm::io::read::<Vec<u8>>();

    let computed_hash = bitcoin_core_rs::sha256(&preimage);
    assert_eq!(hash, computed_hash);

    // Encode the public values of the program.
    let mut bytes = hash.to_vec();
    bytes.extend(preimage);

    // Commit to the public values of the program. The final proof will have a commitment to all the
    // bytes that were committed to.
    sp1_zkvm::io::commit_slice(&bytes);
}
