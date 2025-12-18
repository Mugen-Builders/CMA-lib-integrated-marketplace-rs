#ifndef CMT_ROLLUPS_H
#define CMT_ROLLUPS_H

#include "../libcmt/abi.h"

// Metadata for rollup state
typedef struct {
    cmt_hash_t epoch_hash;
    cmt_hash_t state_root_hash;
    cmt_hash_t input_root_hash;
    cmt_hash_t voucher_root_hash;
    cmt_hash_t notice_root_hash;
    cmt_hash_t exception_root_hash;
    uint64_t epoch_num;
} cmt_metadata_t;

// Payload (opaque bytes)
typedef struct {
    uint64_t size;
    void *data;
} cmt_payload_t;

// Advance input struct (used in parser)
typedef struct {
    cmt_metadata_t metadata;
    cmt_payload_t payload;
} cmt_rollup_advance_t;

// Enums/constants (from typical Cartesi)
typedef enum {
    CMT_ROLLUP_ADVANCE = 0,
    CMT_ROLLUP_INSPECT = 1
} cmt_rollup_request_type_t;

#endif // CMT_ROLLUPS_H