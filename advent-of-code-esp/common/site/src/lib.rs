use std::collections::BTreeMap;

use advent_of_code_solutions::aoc_solution::AoCSolution;
use common_site_handler::load_and_serve_about_page;
use common_site_handler::load_and_serve_default_page;
use common_site_handler::load_and_server_event_page;
use css_handler::load_and_serve_css;
use esp_idf_svc::http::server::EspHttpServer;
use event_list_handler::load_and_serve_event;
use image_handler::load_and_serve_images;
use js_handler::load_and_serve_js;

mod common_site_handler;
mod css_handler;
mod event_list_handler;
mod globals;
mod image_handler;
mod js_handler;
mod multipart;

pub const BUILD_DATE: &str = env!("BUILD_DATE");
pub const SHORT_COMMIT_HASH: &str = env!("SHORT_COMMIT_HASH");

pub fn site(server: &mut EspHttpServer, aoc: &BTreeMap<u32, BTreeMap<u32, Box<dyn AoCSolution>>>) {
    let _ = load_and_serve_css(server);
    let _ = load_and_serve_js(server);
    let _ = load_and_serve_images(server);
    let _ = load_and_serve_default_page(server);
    let _ = load_and_serve_about_page(server);

    let years: Vec<u32> = aoc.keys().cloned().collect();
    let _ = load_and_server_event_page(server, &years);

    for (year, aoc_year) in aoc {
        let _ = load_and_serve_event(server, *year, aoc_year);
    }
}
