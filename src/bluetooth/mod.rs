#![allow(clippy::module_name_repetitions, dead_code)]

use bluedroid::gatt_server::{Profile, GLOBAL_GATT_SERVER};

mod battery;
mod calibration;
mod current_time;
mod device_information;
mod firmware_upgrade;
mod heart_rate;
mod historic_data;
mod optical_frontend_configuration;
mod pulse_loop;
mod pulse_oximeter;
mod sensor_data;
mod settings;

pub struct BluetoothAPI {
    // pub(crate) pulse_oximeter: pulse_oximeter::PulseOximeterServiceContainer,
    // pub(crate) heart_rate: heart_rate::HeartRateServiceContainer,
    // pub(crate) historic_data: historic_data::HistoricDataServiceContainer,
    // pub(crate) settings: settings::SettingsServiceContainer,
    // pub(crate) device_information: device_information::DeviceInformationServiceContainer,
    // pub(crate) current_time: current_time::CurrentTimeServiceContainer,
    // pub(crate) battery: battery::BatteryServiceContainer,
    pub(crate) pulse_loop: pulse_loop::PulseLoopServiceContainer,
    pub(crate) sensor_data: sensor_data::SensorDataServiceContainer,
    pub(crate) optical_frontend_configuration:
        optical_frontend_configuration::OpticalFrontendConfigurationServiceContainer,
    pub(crate) calibration: calibration::CalibrationServiceContainer,
    // pub(crate) firmware_upgrade: firmware_upgrade::FirmwareUpgradeServiceContainer,
}

impl BluetoothAPI {
    pub fn start(&self) {
        let profile = Profile::new(0x0001)
            .name("Main profile")
            .service(&self.pulse_loop.service)
            .service(&self.sensor_data.service)
            .service(&self.optical_frontend_configuration.service)
            .build();

        GLOBAL_GATT_SERVER
            .lock()
            .unwrap()
            .device_name("pulse.loop")
            .appearance(bluedroid::utilities::Appearance::WristWornPulseOximeter)
            .advertise_service(&self.pulse_loop.service)
            .profile(profile)
            .start();
    }

    pub fn initialise() -> Self {
        // let pulse_oximeter = pulse_oximeter::PulseOximeterServiceContainer::initialise();
        // let heart_rate = heart_rate::HeartRateServiceContainer::initialise();
        // let historic_data = historic_data::HistoricDataServiceContainer::initialise();
        // let settings = settings::SettingsServiceContainer::initialise();
        // let device_information = device_information::DeviceInformationServiceContainer::initialise();
        // let current_time = current_time::CurrentTimeServiceContainer::initialise();
        // let battery = battery::BatteryServiceContainer::initialise();
        let pulse_loop = pulse_loop::PulseLoopServiceContainer::initialise();
        let sensor_data = sensor_data::SensorDataServiceContainer::initialise();
        let optical_frontend_configuration = optical_frontend_configuration::OpticalFrontendConfigurationServiceContainer::initialise();
        let calibration = calibration::CalibrationServiceContainer::initialise();
        // let firmware_upgrade = firmware_upgrade::FirmwareUpgradeServiceContainer::initialise();

        Self {
            pulse_loop,
            sensor_data,
            optical_frontend_configuration,
            calibration,
        }
    }
}
