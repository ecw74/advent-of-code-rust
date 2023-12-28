use anyhow::{bail, Result};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::peripheral,
    sys::ESP_ERR_NVS_NEW_VERSION_FOUND,
    sys::ESP_ERR_NVS_NO_FREE_PAGES,
    sys::ESP_OK,
    sys::nvs_flash_erase,
    sys::nvs_flash_init,
    wifi::{AuthMethod, BlockingWifi, ClientConfiguration, Configuration, EspWifi},
};
use log::info;

#[derive(Debug)]
pub struct EspError {
    code: i32,
}

impl EspError {
    pub fn code(&self) -> i32 {
        self.code
    }
}

pub fn err(err: i32) -> Result<(), EspError> {
    if err != ESP_OK as i32 {
        return Err(err.into());
    } else {
        return Ok(());
    }
}

impl From<i32> for EspError {
    fn from(e: i32) -> Self {
        EspError { code: e }
    }
}

fn nvs_init() -> Result<(), EspError> {
    unsafe {
        let mut ret = nvs_flash_init();
        if ret == ESP_ERR_NVS_NO_FREE_PAGES as i32 || ret == ESP_ERR_NVS_NEW_VERSION_FOUND as i32 {
            info!("Need to erase flash: rc = {}", ret);
            err(nvs_flash_erase())?;
            ret = nvs_flash_init();
        }
        err(ret)
    }
}

pub fn wifi(
    ssid: &str,
    pass: &str,
    modem: impl peripheral::Peripheral<P=esp_idf_svc::hal::modem::Modem> + 'static,
    sysloop: EspSystemEventLoop,
) -> Result<Box<EspWifi<'static>>> {
    let mut auth_method = AuthMethod::WPA2Personal;
    if ssid.is_empty() {
        bail!("Missing WiFi name")
    }
    if pass.is_empty() {
        auth_method = AuthMethod::None;
        info!("Wifi password is empty");
    }

    nvs_init().expect("Failed to init NVS");

    let mut esp_wifi = EspWifi::new(modem, sysloop.clone(), None)?;

    let mut wifi = BlockingWifi::wrap(&mut esp_wifi, sysloop)?;

    wifi.set_configuration(&Configuration::Client(ClientConfiguration::default()))?;

    info!("Starting wifi...");

    wifi.start()?;

    info!("Scanning...");

    let ap_infos = wifi.scan()?;

    let ours = ap_infos.into_iter().find(|a| a.ssid == ssid);

    let channel = if let Some(ours) = ours {
        info!(
            "Found configured access point {} on channel {}",
            ssid, ours.channel
        );
        Some(ours.channel)
    } else {
        info!(
            "Configured access point {} not found during scanning, will go with unknown channel",
            ssid
        );
        None
    };

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: ssid.into(),
        password: pass.into(),
        channel,
        auth_method,
        ..Default::default()
    }))?;

    info!("Connecting wifi...");

    wifi.connect()?;

    info!("Waiting for DHCP lease...");

    wifi.wait_netif_up()?;

    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;

    info!("Wifi DHCP info: {:?}", ip_info);

    Ok(Box::new(esp_wifi))
}
