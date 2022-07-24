use crate::bluetooth::configuration::GLOBAL_CONFIGURATION;
use esp_idf_sys::*;

/// GAP event handler.
///
/// # Arguments
///
/// * `event`: Event identifier.
/// * `param`: Generic event parameters.
///
/// returns: ()
pub unsafe extern "C" fn gap_event_handler(
    event: esp_gap_ble_cb_event_t,
    param: *mut esp_ble_gap_cb_param_t,
) {
    #[allow(non_upper_case_globals)]
    match event {
        esp_gap_ble_cb_event_t_ESP_GAP_BLE_ADV_DATA_SET_COMPLETE_EVT => {
            log::info!("Handling advertisement data set complete event.");
            log::info!("Starting advertising.");
            esp_nofail!(esp_ble_gap_start_advertising(leaky_box_raw!(
                GLOBAL_CONFIGURATION.advertising_parameters
            ),));
        }
        esp_gap_ble_cb_event_t_ESP_GAP_BLE_SCAN_RSP_DATA_SET_COMPLETE_EVT => {
            log::info!("Handling scan response data set complete event.");
        }
        esp_gap_ble_cb_event_t_ESP_GAP_BLE_ADV_START_COMPLETE_EVT => {
            log::info!("Handling advertisement startup completion.");
            if (*param).adv_start_cmpl.status != esp_bt_status_t_ESP_BT_STATUS_SUCCESS {
                log::error!("Advertising startup failed.");
            }
        }
        _ => {
            log::warn!("Unhandled event #{}.", event);
        }
    };
}
