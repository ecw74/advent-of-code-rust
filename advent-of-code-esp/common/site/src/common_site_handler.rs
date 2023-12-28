use anyhow::Result;
use askama::Template;
use embedded_svc::{http::Method, io::Write};
use embedded_svc::http::headers::location;
use esp_idf_svc::http::server::EspHttpServer;

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate<'a> {
    current_year: &'a u32
}

#[derive(Template)]
#[template(path = "event-list.html")]
struct EventsTemplate<'a> {
    current_year: &'a u32,
    years: &'a Vec<u32>,
}

pub fn load_and_serve_default_page(server: &mut EspHttpServer) -> Result<()> {
    let current_year = crate::globals::CURRENT_YEAR.lock().clone();
    server.fn_handler("/", Method::Get, move |request| {
        let mut response = request.into_response(301, Some("Moved Permanently"), &[location(format!("/{}", current_year).as_str())])?;
        response.write_all("Moved Permanently".as_bytes())?;
        Ok(())
    })?;
    Ok(())
}

pub fn load_and_serve_about_page(server: &mut EspHttpServer) -> Result<()> {
    let current_year = crate::globals::CURRENT_YEAR.lock().clone();

    let about = AboutTemplate {
        current_year: &current_year,
    };
    let about_string = about.render().unwrap().clone();
    server.fn_handler("/about", Method::Get, move |request| {
        let mut response = request.into_ok_response()?;
        response.write_all(about_string.as_bytes())?;
        Ok(())
    })?;
    Ok(())
}

pub fn load_and_server_event_page(server: &mut EspHttpServer, years: &Vec<u32>) -> Result<()> {
    let current_year = crate::globals::CURRENT_YEAR.lock().clone();
    let events = EventsTemplate {
        current_year: &current_year,
        years: &years,
    };
    let events_string = events.render().unwrap().clone();
    server.fn_handler("/events", Method::Get, move |request| {
        let mut response = request.into_ok_response()?;
        response.write_all(events_string.as_bytes())?;
        Ok(())
    })?;

    Ok(())
}
