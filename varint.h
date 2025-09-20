#ifndef VARINT_H
#define VARINT_H

size_t encode_varint(uint64_t, uint8_t *);
uint64_t decode_varint(const uint8_t **);

#endif /* VARINT_H */
