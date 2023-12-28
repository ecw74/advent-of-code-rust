use embedded_svc::{http::headers::content_type, http::Method, io::Write};
use esp_idf_svc::http::server::EspHttpServer;

pub fn load_and_serve_css(server: &mut EspHttpServer) -> anyhow::Result<()> {
    server.fn_handler("/static/style.css", Method::Get, |request| {
        let css_style_bytes: &'static [u8] = include_bytes!("../css/style.css");
        let mut response = request.into_response(200, Some("OK"), &[content_type("text/css")])?;
        response.write_all(css_style_bytes)?;
        Ok(())
    })?;

    server.fn_handler("/static/highcontrast.css", Method::Get, |request| {
        let css_style_bytes: &'static [u8] = include_bytes!("../css/highcontrast.css");
        let mut response = request.into_response(200, Some("OK"), &[content_type("text/css")])?;
        response.write_all(css_style_bytes)?;
        Ok(())
    })?;

    server.fn_handler("/static/prism.css", Method::Get, |request| {
        let css_style_bytes: &'static [u8] = include_bytes!("../css/prism.css");
        let mut response = request.into_response(200, Some("OK"), &[content_type("text/css")])?;
        response.write_all(css_style_bytes)?;
        Ok(())
    })?;

    server.fn_handler("/static/tree.css", Method::Get, |request| {
        let css_style_bytes: &'static [u8] = include_bytes!("../css/tree.css");
        let mut response = request.into_response(200, Some("OK"), &[content_type("text/css")])?;
        response.write_all(css_style_bytes)?;
        Ok(())
    })?;

    Ok(())
}
