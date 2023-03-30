# Custom types

Here are described the custom data types used in BLE communication. Data is sent as a byte buffer and all values are serialised in little-endian order.

## Raw data

A custom type that contains the ambient and LEDs readings from the frontend in microvolts.

### Format

| Field                 | Type  | Length  |
| --------------------- | ----- | ------- |
| Ambient phase reading | `i32` | 4 bytes |
| LED1 phase reading    | `i32` | 4 bytes |
| LED2 phase reading    | `i32` | 4 bytes |
| LED3 phase reading    | `i32` | 4 bytes |

## Filtered data

A custom type that contains the DC and AC filtered values in microvolts.

### Format

| Field   | Type  | Length  |
| ------- | ----- | ------- |
| LED1 DC | `i32` | 4 bytes |
| LED1 AC | `i32` | 4 bytes |
| LED2 DC | `i32` | 4 bytes |
| LED2 AC | `i32` | 4 bytes |
| LED3 DC | `i32` | 4 bytes |
| LED3 AC | `i32` | 4 bytes |

