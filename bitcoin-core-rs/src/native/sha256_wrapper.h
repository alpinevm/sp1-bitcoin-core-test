#ifndef SHA256_WRAPPER_H
#define SHA256_WRAPPER_H

#ifdef __cplusplus
extern "C"
{
#endif

    // Perform SHA256 hash on input data
    // input: pointer to input data
    // input_len: length of input data in bytes
    // output: pointer to output buffer (must be at least 32 bytes)
    void sha256_hash(const unsigned char *input, unsigned int input_len, unsigned char output[32]);

#ifdef __cplusplus
}
#endif

#endif // SHA256_WRAPPER_H
