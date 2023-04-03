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

## Capacitor value

A custom type that represents the possible values of the frontend capacitors.
The value is encoded as follows.

### Encoding

| Value | Capacitor |
| ----- | --------- |
| 0     | 5 pF      |
| 1     | 2.5 pF    |
| 2     | 10 pF     |
| 3     | 7.5 pF    |
| 4     | 20 pF     |
| 5     | 17.5 pF   |
| 6     | 25 pF     |
| 7     | 22.5 pF   |

## Resistor value

A custom type that represents the possible values of the frontend resistors.
The value is encoded as follows.

### Encoding

| Value | Resistor |
| ----- | -------- |
| 0     | 500 kOhm |
| 1     | 250 kOhm |
| 2     | 100 kOhm |
| 3     | 50 kOhm  |
| 4     | 25 kOhm  |
| 5     | 10 kOhm  |
| 6     | 1 MOhm   |
| 7     | 2 MOhm   |