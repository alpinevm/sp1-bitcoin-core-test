#include "sha256_wrapper.h"
#include "vendor/bitcoin/src/crypto/sha256.h"

extern "C" void sha256_hash(const unsigned char *input, unsigned int input_len, unsigned char output[32])
{
    CSHA256 sha256;
    sha256.Write(input, input_len);
    sha256.Finalize(output);
}
