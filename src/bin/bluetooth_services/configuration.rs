// use esp_idf_sys::*;

// use bluedroid::Configuration;

// impl Default for Configuration {
//     fn default() -> Self {
//         const APPEARANCE: i32 = ESP_BLE_APPEARANCE_PULSE_OXIMETER_WRIST as i32;
//         const MIN_INTERVAL_MS: f32 = 7.5; // Slave connection minimum interval
//         const MAX_INTERVAL_MS: f32 = 12.5; // Slave connection maximum interval

//         Self {
//             advertising_data: esp_ble_adv_data_t {
//                 set_scan_rsp: false,
//                 include_name: true,
//                 include_txpower: true,
//                 min_interval: ((MIN_INTERVAL_MS / 1.25) as u16).into(),
//                 max_interval: ((MAX_INTERVAL_MS / 1.25) as u16).into(),
//                 appearance: APPEARANCE,
//                 manufacturer_len: 0,
//                 p_manufacturer_data: std::ptr::null_mut(),
//                 service_data_len: 0,
//                 p_service_data: std::ptr::null_mut(),
//                 service_uuid_len: 0,
//                 p_service_uuid: std::ptr::null_mut(),
//                 flag: (ESP_BLE_ADV_FLAG_GEN_DISC | ESP_BLE_ADV_FLAG_BREDR_NOT_SPT) as u8,
//             },
//             scan_response_data: esp_ble_adv_data_t {
//                 set_scan_rsp: true,
//                 include_name: true,
//                 include_txpower: true,
//                 min_interval: ((MIN_INTERVAL_MS / 1.25) as u16).into(), // Slave connection min interval, Time = min_interval * 1.25 msec
//                 max_interval: ((MAX_INTERVAL_MS / 1.25) as u16).into(), // Slave connection max interval, Time = max_interval * 1.25 msec
//                 appearance: APPEARANCE,
//                 manufacturer_len: 0,
//                 p_manufacturer_data: std::ptr::null_mut(),
//                 service_data_len: 0,
//                 p_service_data: std::ptr::null_mut(),
//                 service_uuid_len: 0,
//                 p_service_uuid: std::ptr::null_mut(),
//                 flag: (ESP_BLE_ADV_FLAG_GEN_DISC | ESP_BLE_ADV_FLAG_BREDR_NOT_SPT) as u8,
//             },
//             gatt_db: vec![
//                 // Service Declaration: Device Information
//                 esp_gatts_attr_db_t {
//                     attr_control: esp_attr_control_t {
//                         auto_rsp: ESP_GATT_AUTO_RSP as u8,
//                     },
//                     att_desc: esp_attr_desc_t {
//                         uuid_length: ESP_UUID_LEN_16 as u16,
//                         uuid_p: Configuration::esp_uuid_as_u8_ptr(ESP_GATT_UUID_PRI_SERVICE),
//                         perm: ESP_GATT_PERM_READ as u16,
//                         max_length: std::mem::size_of_val(&ESP_GATT_UUID_DEVICE_INFO_SVC) as u16,
//                         length: std::mem::size_of_val(&ESP_GATT_UUID_DEVICE_INFO_SVC) as u16,
//                         value: Configuration::esp_uuid_as_u8_ptr(ESP_GATT_UUID_DEVICE_INFO_SVC),
//                     },
//                 },
//                 // Characteristic Declaration: Manufacturer Name String
//                 esp_gatts_attr_db_t {
//                     attr_control: esp_attr_control_t {
//                         auto_rsp: ESP_GATT_AUTO_RSP as u8,
//                     },
//                     att_desc: esp_attr_desc_t {
//                         uuid_length: ESP_UUID_LEN_16 as u16,
//                         uuid_p: Configuration::esp_uuid_as_u8_ptr(ESP_GATT_UUID_CHAR_DECLARE),
//                         perm: ESP_GATT_PERM_READ as u16,
//                         max_length: std::mem::size_of_val(&ESP_GATT_UUID_MANU_NAME) as u16,
//                         length: std::mem::size_of_val(&ESP_GATT_UUID_MANU_NAME) as u16,
//                         value: Configuration::esp_uuid_as_u8_ptr(ESP_GATT_UUID_MANU_NAME),
//                     },
//                 },
//                 // Characteristic Value: Manufacturer Name String
//                 esp_gatts_attr_db_t {
//                     attr_control: esp_attr_control_t {
//                         auto_rsp: ESP_GATT_AUTO_RSP as u8,
//                     },
//                     att_desc: esp_attr_desc_t {
//                         uuid_length: ESP_UUID_LEN_16 as u16,
//                         uuid_p: Configuration::esp_uuid_as_u8_ptr(ESP_GATT_UUID_MANU_NAME),
//                         perm: ESP_GATT_PERM_READ as u16,
//                         max_length: std::mem::size_of_val("pulse.loop") as u16,
//                         length: std::mem::size_of_val("pulse.loop") as u16,
//                         value: "pulse.loop".as_ptr() as *mut u8,
//                     },
//                 },
//             ],
//         }
//     }
// }
