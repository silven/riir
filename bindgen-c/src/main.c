#include "bindings.h"

int main() {
    const uint8_t* input = get_sample_data();
    size_t input_size = get_sample_data_size();

    const uint8_t* compressed_output = malloc(input_size);
    const uint8_t* decompressed_output = malloc(input_size);

    uint8_t compression_error = compress(input, compressed_output, input_size);

    if (compression_error != 0) {
        printf("Error during compression!");
        return 1;
    }

    uint8_t decompression_error = decompress(compressed_output, decompressed_output, input_size);
    if (decompression_error != 0) {
        printf("Error during decompression!");
        return 1;
    }

    uint32_t cmp_result = memcmp(input, decompressed_output, input_size);
    if (cmp_result == 0) {
        printf("\nOk!\n");
    } else {
        printf("\nDecompressed data does not equals input!\n");
    }

    free(compressed_output);
    free(decompressed_output);

    return 0;
}