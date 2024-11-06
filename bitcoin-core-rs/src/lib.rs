const SHA256_OUTPUT_SIZE: usize = 32;

extern "C" {
    fn sha256_hash(input: *const u8, input_len: u32, output: *mut u8);
}

pub fn sha256(input: &[u8]) -> [u8; SHA256_OUTPUT_SIZE] {
    let mut output = [0u8; SHA256_OUTPUT_SIZE];
    unsafe {
        sha256_hash(input.as_ptr(), input.len() as u32, output.as_mut_ptr());
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn test_sha256() {
        let test_bytes = hex!("deadbeef");
        let hash = sha256(&test_bytes);
        assert_eq!(
            hash,
            hex!("5f78c33274e43fa9de5659265c1d917e25c03722dcb0b8d27db8d5feaa813953")
        );
    }

    #[test]
    fn test_sha256_empty() {
        let hash = sha256(&[]);
        assert_eq!(
            hash,
            hex!("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
        );
    }
}
