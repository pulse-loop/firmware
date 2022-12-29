# pulse.loop

An open-source pulse oximeter built with an ESP32C3.

## Bluetooth interface

The pulse.loop uses Bluetooth LE to communicate with client devices.
It uses as many standardised services as possible, and implements some extra services for internal debugging and configuration.

### Standardised services

The specifications for the standardised services are specified by the Bluetooth SIG.

Here is a list of the SIG-defined services used by the pulse.loop:

| Service            | UUID     | Description                                  |
|--------------------|----------|----------------------------------------------|
| Battery            | `0x180F` | Battery level                                |
| Current Time       | `0x1805` | Current time                                 |
| Device Information | `0x180A` | Manufacturer, model, serial number, ...      |
| Heart Rate         | `0x180D` | Heart rate measurement, body sensor location |
| Pulse Oximeter     | `0x1822` | Pulse oximeter measurement                   |

### Custom services

We also use some custom services for internal debugging and configuration.

| Service                        | UUID                                   | Description                                                                |
|--------------------------------|----------------------------------------|----------------------------------------------------------------------------|
| Firmware upgrade               | `0BA1B4AC-734A-4E75-AD22-8D5BBDEA5025` | Firmware upgrade service                                                   |
| Historic data                  | `DE753059-8906-4F07-A192-12879BB84DA7` | Historic data that can be downloaded by the user                           |
| Optical frontend configuration | `C8F276D4-E0DD-4660-8070-619FF734134B` | `[DEBUG ONLY]` Optical sensor configuration                                |
| Raw sensor data                | `272DF1F7-9D28-4B8C-86F6-30DB30ACE42C` | `[DEBUG ONLY]` Optical sensor data, IMU data, system status and parameters |
| Settings                       | `821198C8-3036-4E14-B01C-364F2B20C603` | Settings that can be changed by the user                                   |
| pulse.loop identifier          | `68D68245-CFD8-4A1C-9858-B27ABC4C382E` | pulse.loop BLE API version. Used for detection.                            |

#### pulse.loop identifier

This service is used to detect if a device is a pulse.loop or not. It contains a single characteristic, which is a read-only string containing the BLE API version.
This service is advertised in scan response data, so it is not necessary to connect to the device to detect it.

| Characteristic | Access | UUID                                   | Description      | FW  | SW |
|----------------|--------|----------------------------------------|------------------|-----|----|
| Version        | Read   | `1852299D-AE64-4E4F-B915-CB37E7FD57C9` | BLE API version. | Yes | No |

#### Settings

The settings service is used to change the settings of the pulse.loop. It exposes high-level settings that can be changed by the user.

| Characteristic | Access | UUID | Description |
|----------------|--------|------|-------------|

#### Historic data

The historic data service is used to download historic data from the pulse.loop.

| Characteristic | Access | UUID | Description |
|----------------|--------|------|-------------|

#### Firmware upgrade

The firmware upgrade service is used to upgrade the firmware of the pulse.loop. It exposes a special serial port over BLE that can be used to send firmware images to the pulse.loop.

| Characteristic | Access | UUID | Description |
|----------------|--------|------|-------------|

#### Raw sensor data

Data from the optical frontend and other sensors.

| Characteristic        | Access | Type  | UUID                                   | Description                                                             | FW  | SW  |
|-----------------------|--------|-------|----------------------------------------|-------------------------------------------------------------------------|-----|-----|
| Ambient phase reading | Read   | `f32` | `33EAF25F-7A5C-4327-A95B-B602DA54C443` | The latest reading during the ambient phase [V].                        | Yes | Yes |
| AVG LED1 - Ambient    | Read   | `f32` | `0237DF64-3110-48CA-B3C8-82DF0F7A6A91` | The averaged reading during the LED1 phase minus the ambient phase [V]. | No  | No  |
| LED1 - Ambient        | Read   | `f32` | `CF66D344-584D-4E67-AC30-17D28B099A30` | The latest reading during the LED1 phase minus the ambient phase [V].   | Yes | Yes |
| LED1 ADC reading      | Read   | `f32` | `05500B81-516D-4BD9-95BA-C0B87C911DDB` | The latest reading during the LED1 phase [V].                           | Yes | Yes |
| LED2 ADC reading      | Read   | `f32` | `A93B639D-8A8D-43EA-8A5A-8175D7C09E0B` | The latest reading during the LED2 phase [V].                           | Yes | Yes |
| LED3 ADC reading      | Read   | `f32` | `C0A12246-79E4-4BD7-8A4F-B841D5590F70` | The latest reading during the LED3 phase [V].                           | Yes | Yes |

#### Optical frontend configuration

Analog frontend parameter configuration for testing and algorithm development.

| Characteristic               | Access     | Type  | UUID                                   | Description                                      | FW  | SW  |
|------------------------------|------------|-------|----------------------------------------|--------------------------------------------------|-----|-----|
| ADC averages                 | Read/Write | `u8`  | `7ADE19EA-2202-48E1-AFFB-4D8504024C37` | The number of averages performed by the ADC [-]. | Yes | No  |
| Ambient ADC conversion end   | Read/Write | `f32` | `9B6AF28C-9558-49ED-844B-06E7B8B0E6C3` | The end time of ambient ADC conversion [µs].     | Yes | Yes |
| Ambient ADC conversion start | Read/Write | `f32` | `66DC5EDA-B89E-43D5-B940-13E29A468C77` | The start time of ambient ADC conversion [µs].   | Yes | Yes |
| Ambient ADC reset end        | Read/Write | `f32` | `E9AB33D4-DA9C-4424-851A-16CF66AA08C0` | The end time of ambient ADC reset [µs].          | Yes | Yes |
| Ambient ADC reset start      | Read/Write | `f32` | `FD7FAFE2-4464-4F8C-A55C-79E45FB916B3` | The start time of ambient ADC reset [µs].        | Yes | Yes |
| Ambient offset current       | Read/Write | `f32` | `4ED69FED-8261-4931-A8A4-CA67B406A73A` | The offset current of ambient [µA].              | Yes | Yes |
| Ambient sample end           | Read/Write | `f32` | `83C29A09-B658-4316-A1FD-D8AD97C02F80` | The end time of ambient sample [µs].             | Yes | Yes |
| Ambient sample start         | Read/Write | `f32` | `C35EBCC5-BCA4-4716-90E3-37B94D9AD6FF` | The start time of ambient sample [µs].           | Yes | Yes |
| Decimation factor            | Read/Write | `u8`  | `4D5A0E9C-0164-4D65-8F2D-86741B820EEF` | The number of data samples to be averaged [-].   | Yes | No  |
| Dynamic power-down end       | Read/Write | `f32` | `BC276997-792F-4391-9371-78F1C1038DB7` | The end time of dynamic power-down [µs].         | Yes | Yes |
| Dynamic power-down start     | Read/Write | `f32` | `0B68331C-B628-4D81-BBDB-47B79EA2430E` | The start time of dynamic power-down [µs].       | Yes | Yes |
| LED1 ADC conversion end      | Read/Write | `f32` | `C455472B-4231-4EF7-A3BD-A1AE2676F9D2` | The end time of LED1 ADC conversion [µs].        | Yes | Yes |
| LED1 ADC conversion start    | Read/Write | `f32` | `ED9B9EE3-AAFE-4034-8C07-1D7F495288B1` | The start time of LED1 ADC conversion [µs].      | Yes | Yes |
| LED1 ADC reset end           | Read/Write | `f32` | `7049E8C6-A0CE-4380-8186-1B7CD34179ED` | The end time of LED1 ADC reset [µs].             | Yes | Yes |
| LED1 ADC reset start         | Read/Write | `f32` | `C8B42A6D-ECFC-40E8-8E3C-F5876EE749A3` | The start time of LED1 ADC reset [µs].           | Yes | Yes |
| LED1 current                 | Read/Write | `f32` | `A20B7943-5E1D-4053-8C4E-CD44463F460D` | The current of LED1 [mA].                        | Yes | Yes |
| LED1 lighting end            | Read/Write | `f32` | `9C678B37-F3AA-4B8C-AFD5-10B4694E49C7` | The end time of LED1 illumination [µs].          | Yes | Yes |
| LED1 lighting start          | Read/Write | `f32` | `F02C15DF-14F1-4872-BE99-33EE64F0E0B3` | The start time of LED1 illumination [µs].        | Yes | Yes |
| LED1 offset current          | Read/Write | `f32` | `C5C6B835-56A6-4FC5-81BF-7512595DF3BD` | The offset current of LED1 [µA].                 | Yes | Yes |
| LED1 sample end              | Read/Write | `f32` | `F60A8B03-FAB1-433D-9D9E-8722DF003329` | The end time of LED1 sample [µs].                | Yes | Yes |
| LED1 sample start            | Read/Write | `f32` | `FB219512-DC81-461A-B558-FE6E310E9333` | The start time of LED1 sample [µs].              | Yes | Yes |
| LED2 ADC conversion end      | Read/Write | `f32` | `40314C89-306E-47F0-AE1F-C5DDD8C0CDDD` | The end time of LED2 ADC conversion [µs].        | Yes | Yes |
| LED2 ADC conversion start    | Read/Write | `f32` | `160CC306-3CA6-4BF5-AC0B-85443F3CFC6B` | The start time of LED2 ADC conversion [µs].      | Yes | Yes |
| LED2 ADC reset end           | Read/Write | `f32` | `34D6F164-543F-49F4-B0F1-6E68DC4CEE6B` | The end time of LED2 ADC reset [µs].             | Yes | Yes |
| LED2 ADC reset start         | Read/Write | `f32` | `E34424D5-68DA-467F-93FE-BE49F19FAB0E` | The start time of LED2 ADC reset [µs].           | Yes | Yes |
| LED2 current                 | Read/Write | `f32` | `29CA51A3-B33B-44FD-853C-00FE8827ADC4` | The current of LED2 [mA].                        | Yes | Yes |
| LED2 lighting end            | Read/Write | `f32` | `B85968BA-FB52-46E8-81A5-0F837BF3D6EB` | The end time of LED2 illumination [µs].          | Yes | Yes |
| LED2 lighting start          | Read/Write | `f32` | `F710D5DC-2655-42D6-97AA-7A5FDF0285C8` | The start time of LED2 illumination [µs].        | Yes | Yes |
| LED2 offset current          | Read/Write | `f32` | `1F23AD86-30CB-4AC2-AD23-226DA5B2EB0C` | The offset current of LED2 [µA].                 | Yes | Yes |
| LED2 sample end              | Read/Write | `f32` | `F752142C-5BFC-4274-9044-E81D3F2F274A` | The end time of LED2 sample [µs].                | Yes | Yes |
| LED2 sample start            | Read/Write | `f32` | `38644B85-3D2E-4D31-9679-06C9EB6BAC2D` | The start time of LED2 sample [µs].              | Yes | Yes |
| LED3 ADC conversion end      | Read/Write | `f32` | `7C2A9A6F-95EB-45ED-B7E1-BB290F7853ED` | The end time of LED3 ADC conversion [µs].        | Yes | Yes |
| LED3 ADC conversion start    | Read/Write | `f32` | `C03D3143-E6B6-49AB-85FC-EEED3A43B530` | The start time of LED3 ADC conversion [µs].      | Yes | Yes |
| LED3 ADC reset end           | Read/Write | `f32` | `A7D441AA-C456-4CBF-A0B9-84DBF33934EF` | The end time of LED3 ADC reset [µs].             | Yes | Yes |
| LED3 ADC reset start         | Read/Write | `f32` | `536D72C8-DFF0-4E38-93F7-7F376316EA8D` | The start time of LED3 ADC reset [µs].           | Yes | Yes |
| LED3 current                 | Read/Write | `f32` | `F7535ED9-CB9F-469A-817E-1635DC3B68B0` | The current of LED3 [mA].                        | Yes | Yes |
| LED3 lighting end            | Read/Write | `f32` | `5B7F9859-092B-43D4-AC6B-AC9DD4742AB2` | The end time of LED3 illumination [µs].          | Yes | Yes |
| LED3 lighting start          | Read/Write | `f32` | `0B098015-110E-487E-AAE9-BEA1ED1F54A0` | The start time of LED3 illumination [µs].        | Yes | Yes |
| LED3 offset current          | Read/Write | `f32` | `41AE7B18-F5D7-4475-9E3F-49354F077CED` | The offset current of LED3 [µA].                 | Yes | Yes |
| LED3 sample end              | Read/Write | `f32` | `249782EC-004B-4A3D-9608-5143E69AB294` | The end time of LED3 sample [µs].                | Yes | Yes |
| LED3 sample start            | Read/Write | `f32` | `733C5AED-D3B3-4F65-8898-6EA37DA30F71` | The start time of LED3 sample [µs].              | Yes | Yes |
| TIA capacitor 1              | Read/Write | `f32` | `08B3B8E9-D3AD-48EB-B93B-AF4D3695F05C` | The value of TIA capacitor 1 [pF].               | Yes | Yes |
| TIA capacitor 2              | Read/Write | `f32` | `740669DF-57D3-4147-87B4-DC302512F20A` | The value of TIA capacitor 2 [pF].               | Yes | Yes |
| TIA resistor 1               | Read/Write | `f32` | `81831E3A-917E-4252-9C16-42BA8FF3F47A` | The value of TIA resistor 1 [Ohm].               | Yes | Yes |
| TIA resistor 2               | Read/Write | `f32` | `A3F694D1-C378-4124-BF56-468DFAFF14E6` | The value of TIA resistor 2 [Ohm].               | Yes | Yes |
| Total window length          | Read/Write | `f32` | `B904BD23-6082-4507-8BD2-7333EF6A2726` | The total length of the windows [µs].            | Yes | Yes |
