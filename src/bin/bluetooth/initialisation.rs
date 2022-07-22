//! Initialisation function for BLE Server.

use esp_idf_sys::{
    esp_ble_gap_register_callback, esp_ble_gatts_app_register, esp_ble_gatts_register_callback,
    esp_bluedroid_enable, esp_bluedroid_init, esp_bt_controller_config_t, esp_bt_controller_enable,
    esp_bt_controller_init, esp_bt_mode_t_ESP_BT_MODE_BLE, esp_nofail, nvs_flash_init,
    AGC_RECORRECT_EN, BLE_HW_TARGET_CODE_ESP32C3_CHIP_ECO0, CFG_NASK,
    CONFIG_BT_CTRL_ADV_DUP_FILT_MAX, CONFIG_BT_CTRL_BLE_MAX_ACT_EFF,
    CONFIG_BT_CTRL_BLE_STATIC_ACL_TX_BUF_NB, CONFIG_BT_CTRL_CE_LENGTH_TYPE_EFF,
    CONFIG_BT_CTRL_COEX_PHY_CODED_TX_RX_TLIM_EFF, CONFIG_BT_CTRL_DFT_TX_POWER_LEVEL_EFF,
    CONFIG_BT_CTRL_HCI_TL_EFF, CONFIG_BT_CTRL_HW_CCA_EFF, CONFIG_BT_CTRL_HW_CCA_VAL,
    CONFIG_BT_CTRL_MODE_EFF, CONFIG_BT_CTRL_PINNED_TO_CORE, CONFIG_BT_CTRL_RX_ANTENNA_INDEX_EFF,
    CONFIG_BT_CTRL_SLEEP_CLOCK_EFF, CONFIG_BT_CTRL_SLEEP_MODE_EFF,
    CONFIG_BT_CTRL_TX_ANTENNA_INDEX_EFF, ESP_BT_CTRL_CONFIG_MAGIC_VAL, ESP_BT_CTRL_CONFIG_VERSION,
    ESP_TASK_BT_CONTROLLER_PRIO, ESP_TASK_BT_CONTROLLER_STACK, MESH_DUPLICATE_SCAN_CACHE_SIZE,
    NORMAL_SCAN_DUPLICATE_CACHE_SIZE, SCAN_DUPLICATE_MODE, SCAN_DUPLICATE_TYPE_VALUE,
    SLAVE_CE_LEN_MIN_DEFAULT,
};

/// Starts the Bluetooth LE stack and initialises it with default values for this device.
pub fn initialise() {
    #[allow(clippy::cast_possible_truncation)]
    let mut default_configuration = esp_bt_controller_config_t {
        magic: ESP_BT_CTRL_CONFIG_MAGIC_VAL,
        version: ESP_BT_CTRL_CONFIG_VERSION,
        controller_task_stack_size: ESP_TASK_BT_CONTROLLER_STACK as u16,
        controller_task_prio: ESP_TASK_BT_CONTROLLER_PRIO as u8,
        controller_task_run_cpu: CONFIG_BT_CTRL_PINNED_TO_CORE as u8,
        bluetooth_mode: CONFIG_BT_CTRL_MODE_EFF as u8,
        ble_max_act: CONFIG_BT_CTRL_BLE_MAX_ACT_EFF as u8,
        sleep_mode: CONFIG_BT_CTRL_SLEEP_MODE_EFF as u8,
        sleep_clock: CONFIG_BT_CTRL_SLEEP_CLOCK_EFF as u8,
        ble_st_acl_tx_buf_nb: CONFIG_BT_CTRL_BLE_STATIC_ACL_TX_BUF_NB as u8,
        ble_hw_cca_check: CONFIG_BT_CTRL_HW_CCA_EFF as u8,
        ble_adv_dup_filt_max: CONFIG_BT_CTRL_ADV_DUP_FILT_MAX as u16,
        coex_param_en: false,
        ce_len_type: CONFIG_BT_CTRL_CE_LENGTH_TYPE_EFF as u8,
        coex_use_hooks: false,
        hci_tl_type: CONFIG_BT_CTRL_HCI_TL_EFF as u8,
        hci_tl_funcs: std::ptr::null_mut(),
        txant_dft: CONFIG_BT_CTRL_TX_ANTENNA_INDEX_EFF as u8,
        rxant_dft: CONFIG_BT_CTRL_RX_ANTENNA_INDEX_EFF as u8,
        txpwr_dft: CONFIG_BT_CTRL_DFT_TX_POWER_LEVEL_EFF as u8,
        cfg_mask: CFG_NASK,
        scan_duplicate_mode: SCAN_DUPLICATE_MODE as u8,
        scan_duplicate_type: SCAN_DUPLICATE_TYPE_VALUE as u8,
        normal_adv_size: NORMAL_SCAN_DUPLICATE_CACHE_SIZE as u16,
        mesh_adv_size: MESH_DUPLICATE_SCAN_CACHE_SIZE as u16,
        coex_phy_coded_tx_rx_time_limit: CONFIG_BT_CTRL_COEX_PHY_CODED_TX_RX_TLIM_EFF as u8,
        hw_target_code: BLE_HW_TARGET_CODE_ESP32C3_CHIP_ECO0,
        slave_ce_len_min: SLAVE_CE_LEN_MIN_DEFAULT as u8,
        hw_recorrect_en: AGC_RECORRECT_EN as u8,
        cca_thresh: CONFIG_BT_CTRL_HW_CCA_VAL as u8,
    };

    log::info!("Initialising Bluetooth LE stack.");

    unsafe {
        esp_nofail!(nvs_flash_init());
        esp_nofail!(esp_bt_controller_init(&mut default_configuration));
        esp_nofail!(esp_bt_controller_enable(esp_bt_mode_t_ESP_BT_MODE_BLE));
        esp_nofail!(esp_bluedroid_init());
        esp_nofail!(esp_bluedroid_enable());
        esp_nofail!(esp_ble_gatts_register_callback(Some(
            crate::bluetooth::gatts_event_handler::gatts_event_handler
        )));
        esp_nofail!(esp_ble_gap_register_callback(Some(
            crate::bluetooth::gap_event_handler::gap_event_handler
        )));
        esp_nofail!(esp_ble_gatts_app_register(0x55));
    }
}
