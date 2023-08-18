#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


/**
 * Size of the Deoxys-II-256-128 key in bytes.
 */
#define KEY_SIZE 32

/**
 * Size of the nonce in bytes.
 */
#define NONCE_SIZE 15

/**
 * Size of the authentication tag in bytes.
 */
#define TAG_SIZE 16

typedef struct ByteBuffer {
  const uint8_t *ptr;
  size_t len;
  size_t cap;
  const char *err;
} ByteBuffer;

struct ByteBuffer rust_call(const uint8_t *data, size_t len);

void rust_free(struct ByteBuffer byte_buffer);
