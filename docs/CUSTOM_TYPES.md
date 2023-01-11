# Custom types

## Aggregated data

A custom type which contains the LEDs and ambient readings, the calibration upper and lower thresholds for each reading and the type of critical point only for the LEDs.

### Format

| Field                   | Type           | Length  |
|-------------------------|----------------|---------|
| Ambient phase reading   | `f32`          | 4 bytes |
| LED1 ADC reading        | `f32`          | 4 bytes |
| LED2 ADC reading        | `f32`          | 4 bytes |
| LED3 ADC reading        | `f32`          | 4 bytes |
| Ambient lower threshold | `f32`          | 4 bytes |
| Ambient upper threshold | `f32`          | 4 bytes |
| LED1 lower threshold    | `f32`          | 4 bytes |
| LED1 upper threshold    | `f32`          | 4 bytes |
| LED2 lower threshold    | `f32`          | 4 bytes |
| LED2 upper threshold    | `f32`          | 4 bytes |
| LED3 lower threshold    | `f32`          | 4 bytes |
| LED3 upper threshold    | `f32`          | 4 bytes |
| Skip                    | 0              | 2 bits  |
| LED1 critical point     | Critical point | 2 bits  |
| LED2 critical point     | Critical point | 2 bits  |
| LED3 critical point     | Critical point | 2 bits  |

## Critical point

A custom type which could assume three possible values: Maximum, Minimum, None.
