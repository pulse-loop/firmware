use crate::bluetooth::configuration::Configuration;
use esp_idf_sys::{
    esp_ble_gap_cb_param_t, esp_ble_gap_start_advertising, esp_bt_status_t_ESP_BT_STATUS_SUCCESS,
    esp_gap_ble_cb_event_t, esp_gap_ble_cb_event_t_ESP_GAP_BLE_ADV_DATA_SET_COMPLETE_EVT,
    esp_gap_ble_cb_event_t_ESP_GAP_BLE_ADV_START_COMPLETE_EVT, esp_nofail,
};

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
            let mut configuration = Configuration::default();
            esp_nofail!(esp_ble_gap_start_advertising(
                &mut configuration.advertising_parameters
            ));
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
