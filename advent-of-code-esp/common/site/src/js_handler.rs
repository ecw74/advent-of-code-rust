use embedded_svc::{http::headers::content_type, http::Method, io::Write};
use esp_idf_svc::http::server::EspHttpServer;

pub fn load_and_serve_js(server: &mut EspHttpServer) -> anyhow::Result<()> {
    server.fn_handler("/static/prism.js", Method::Get, |request| {
        let css_style_bytes: &'static [u8] = include_bytes!("../js/prism.js");
        let mut response =
            request.into_response(200, Some("OK"), &[content_type("text/javascript")])?;
        response.write_all(css_style_bytes)?;
        Ok(())
    })?;

    Ok(())
}
