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

## Calibration

A custom type that contains the parameters used in the calibration process for the three LEDs.

### Format

| Field                   | Type  | Length  |
| ----------------------- | ----- | ------- |
| LED1 current min        | `i32` | 4 bytes |
| LED1 current max        | `i32` | 4 bytes |
| LED1 offset current min | `i32` | 4 bytes |
| LED1 offset current max | `i32` | 4 bytes |
| LED1 set point          | `i32` | 4 bytes |
| LED1 working threshold  | `i32` | 4 bytes |
| LED1 alpha              | `i32` | 4 bytes |
| LED2 current min        | `i32` | 4 bytes |
| LED2 current max        | `i32` | 4 bytes |
| LED2 offset current min | `i32` | 4 bytes |
| LED2 offset current max | `i32` | 4 bytes |
| LED2 set point          | `i32` | 4 bytes |
| LED2 working threshold  | `i32` | 4 bytes |
| LED2 alpha              | `i32` | 4 bytes |
| LED3 current min        | `i32` | 4 bytes |
| LED3 current max        | `i32` | 4 bytes |
| LED3 offset current min | `i32` | 4 bytes |
| LED3 offset current max | `i32` | 4 bytes |
| LED3 set point          | `i32` | 4 bytes |
| LED3 working threshold  | `i32` | 4 bytes |
| LED3 alpha              | `i32` | 4 bytes |
