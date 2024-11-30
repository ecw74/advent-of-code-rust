use anyhow::Result;
use askama::Template;
use embedded_svc::http::headers::location;
use embedded_svc::{http::Method, io::Write};
use esp_idf_svc::http::server::EspHttpServer;
use esp_idf_svc::io::EspIOError;

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate<'a> {
    current_year: &'a u32,
    image_name: String,
}

#[derive(Template)]
#[template(path = "event-list.html")]
struct EventsTemplate<'a> {
    current_year: &'a u32,
    years: &'a Vec<u32>,
    image_name: String,
}

pub fn load_and_serve_default_page(server: &mut EspHttpServer) -> Result<()> {
    let current_year = crate::globals::CURRENT_YEAR.lock().clone();
    server.fn_handler("/", Method::Get, move |request| -> Result<(), EspIOError> {
        let mut response = request.into_response(
            301,
            Some("Moved Permanently"),
            &[location(format!("/{}", current_year).as_str())],
        )?;
        response.write_all("Moved Permanently".as_bytes())?;
        Ok(())
    })?;
    Ok(())
}

pub fn load_and_serve_about_page(server: &mut EspHttpServer) -> Result<(), EspIOError> {
    let current_year = crate::globals::CURRENT_YEAR.lock().clone();

    let about = AboutTemplate {
        current_year: &current_year,
        image_name: "aoc-logo.avif".to_string()
    };
    let about_string = about.render().unwrap().clone();
    server.fn_handler(
        "/about",
        Method::Get,
        move |request| -> Result<(), EspIOError> {
            let mut response = request.into_ok_response()?;
            response.write_all(about_string.as_bytes())?;
            Ok(())
        },
    )?;
    Ok(())
}

pub fn load_and_server_event_page(server: &mut EspHttpServer, years: &Vec<u32>) -> Result<()> {
    let current_year = crate::globals::CURRENT_YEAR.lock().clone();
    let events = EventsTemplate {
        current_year: &current_year,
        years: &years,
        image_name: "aoc-logo.avif".to_string()
    };
    let events_string = events.render().unwrap().clone();
    server.fn_handler(
        "/events",
        Method::Get,
        move |request| -> Result<(), EspIOError> {
            let mut response = request.into_ok_response()?;
            response.write_all(events_string.as_bytes())?;
            Ok(())
        },
    )?;

    Ok(())
}
