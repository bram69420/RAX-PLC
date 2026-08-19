#ifndef PLC_H
#define PLC_H

#include <stdint.h>

// Returns 0 on success.Returns a negative value on failure.
int32_t plc_init(void);

// Reads a 32-bit value from a PLC address.
int32_t plc_read_u32(uint32_t address, uint32_t *value);

// Writes a 32-bit value to a PLC address.
int32_t plc_write_u32(uint32_t address, uint32_t value);

// Shuts down the PLC connection.
int32_t plc_shutdown(void);

#endif
