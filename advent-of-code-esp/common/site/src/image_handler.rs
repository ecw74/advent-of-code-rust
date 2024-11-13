use embedded_svc::{http::headers::content_type, http::Method, io::Write};
use esp_idf_svc::http::server::EspHttpServer;
use esp_idf_svc::io::EspIOError;

pub fn load_and_serve_images(server: &mut EspHttpServer) -> anyhow::Result<()> {
    server.fn_handler("/favicon.ico", Method::Get, |request| -> Result<(), EspIOError> {
        let favicon_bytes: &'static [u8] = include_bytes!("../img/favicon.ico");
        let mut response = request.into_ok_response()?;
        response.write_all(favicon_bytes)?;
        Ok(())
    })?;

    server.fn_handler("/favicon.png", Method::Get, |request| -> Result<(), EspIOError> {
        let favicon_bytes: &'static [u8] = include_bytes!("../img/favicon.png");
        let mut response = request.into_response(200, Some("OK"), &[content_type("image/png")])?;
        response.write_all(favicon_bytes)?;
        Ok(())
    })?;

    server.fn_handler("/hardware.jpg", Method::Get, |request| -> Result<(), EspIOError> {
        let favicon_bytes: &'static [u8] = include_bytes!("../img/esp32-s3.jpg");
        let mut response = request.into_response(200, Some("OK"), &[content_type("image/jpg")])?;
        response.write_all(favicon_bytes)?;
        Ok(())
    })?;

    Ok(())
}
