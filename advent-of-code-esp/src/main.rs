use std::{thread::sleep, time::Duration};
use std::collections::BTreeMap;

use advent_of_code_solutions::advent_of_code_2022;
use advent_of_code_solutions::advent_of_code_2023;
use advent_of_code_solutions::aoc_solution::AoCSolution;
use anyhow::Result;
use esp_idf_svc::{eventloop::EspSystemEventLoop, hal::prelude::*, http::server::EspHttpServer};
use site::site;
use wifi::wifi;

const STACK_SIZE: usize = 10240;

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // wifi
    let app_config = CONFIG;
    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;
    let _wifi = wifi(
        app_config.wifi_ssid,
        app_config.wifi_psk,
        peripherals.modem,
        sysloop,
    )?;

    let server_configuration = esp_idf_svc::http::server::Configuration {
        stack_size: STACK_SIZE,
        ..Default::default()
    };
    let mut server = EspHttpServer::new(&server_configuration)?;

    let mut aoc: BTreeMap<u32, BTreeMap<u32, Box<dyn AoCSolution>>> = BTreeMap::new();
    let mut aoc_2022: BTreeMap<u32, Box<dyn AoCSolution>> = BTreeMap::new();
    let mut aoc_2023: BTreeMap<u32, Box<dyn AoCSolution>> = BTreeMap::new();

    let _ = advent_of_code_2022(&mut aoc_2022);
    let _ = advent_of_code_2023(&mut aoc_2023);

    aoc.insert(2022, aoc_2022);
    aoc.insert(2023, aoc_2023);

    // Ad here your sites
    site(&mut server, &aoc);

    loop {
        sleep(Duration::from_millis(1000));
    }
}
