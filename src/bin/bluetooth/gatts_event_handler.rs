use esp_idf_sys::{esp_ble_gatts_cb_param_t, esp_gatt_if_t, esp_gatts_cb_event_t};

fn gatts_event_handler(
    event: esp_gatts_cb_event_t,
    gatts_if: esp_gatt_if_t,
    param: *mut esp_ble_gatts_cb_param_t,
) {
}
