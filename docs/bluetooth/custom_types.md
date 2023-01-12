# Custom types

Here are described the custom data types used in BLE communication. Data is sent as a byte buffer and all values are serialised in little-endian order.

## Aggregated data

A custom type that contains the LEDs and ambient phase readings, the calibration upper and lower thresholds for each channel and a flag for indicating critical points on LED channels.

### Format

| Field                   | Type                              | Length  |
|-------------------------|-----------------------------------|---------|
| Ambient phase reading   | `f32`                             | 4 bytes |
| LED1 ADC reading        | `f32`                             | 4 bytes |
| LED2 ADC reading        | `f32`                             | 4 bytes |
| LED3 ADC reading        | `f32`                             | 4 bytes |
| Ambient lower threshold | `f32`                             | 4 bytes |
| Ambient upper threshold | `f32`                             | 4 bytes |
| LED1 lower threshold    | `f32`                             | 4 bytes |
| LED1 upper threshold    | `f32`                             | 4 bytes |
| LED2 lower threshold    | `f32`                             | 4 bytes |
| LED2 upper threshold    | `f32`                             | 4 bytes |
| LED3 lower threshold    | `f32`                             | 4 bytes |
| LED3 upper threshold    | `f32`                             | 4 bytes |
| Padding                 | -                                 | 2 bits  |
| LED1 critical point     | [Critical point](#critical-point) | 2 bits  |
| LED2 critical point     | [Critical point](#critical-point) | 2 bits  |
| LED3 critical point     | [Critical point](#critical-point) | 2 bits  |

## Critical point

A custom type which could assume three possible values: Maximum, Minimum and None.
Each value is coded with two bits.

### Format

| Coding | Value      |
|--------|------------|
| 00     | None       |
| 01     | Minimum    |
| 10     | Maximum    |
| 11     | Do not use |
