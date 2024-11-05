use autocxx::prelude::*;

include_cpp! {
    #include "crypto/sha256.h"
    safety!(unsafe_ffi)
    generate!("CSHA256")
}

pub mod crypto {
    use super::*;

    pub fn sha256(data: &[u8]) -> [u8; 32] {
        let mut hasher = ffi::CSHA256::new().within_unique_ptr();
        unsafe {
            hasher.as_mut().unwrap().Write(data.as_ptr(), data.len());
        }
        let mut hash = [0u8; 32];
        unsafe {
            hasher.as_mut().unwrap().Finalize(hash.as_mut_ptr());
        }
        hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn test_sha256() {
        let test_bytes = hex!("deadbeef");
        let hash = crypto::sha256(&test_bytes);
        assert_eq!(
            hash,
            hex!("5f78c33274e43fa9de5659265c1d917e25c03722dcb0b8d27db8d5feaa813953")
        );
    }

    #[test]
    fn test_sha256_empty() {
        let hash = crypto::sha256(&[]);
        assert_eq!(
            hash,
            hex!("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
        );
    }
}
