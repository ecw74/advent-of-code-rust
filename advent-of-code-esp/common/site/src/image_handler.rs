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


    server.fn_handler("/aoc-2022-5.avif", Method::Get, |request| -> Result<(), EspIOError> {
        let favicon_bytes: &'static [u8] = include_bytes!("/IdeaProjects/advent-of-code-rust/advent-of-code-esp/target/xtensa-esp32s3-espidf/debug/build/site-206c1eb8fbc83c6e/out/build/aoc-2022-5.avif");
        let mut response = request.into_ok_response()?;
        response.write_all(favicon_bytes)?;
        Ok(())
    })?;

    server.fn_handler("/aoc-2023-5.avif", Method::Get, |request| -> Result<(), EspIOError> {
        let favicon_bytes: &'static [u8] = include_bytes!("/IdeaProjects/advent-of-code-rust/advent-of-code-esp/target/xtensa-esp32s3-espidf/debug/build/site-206c1eb8fbc83c6e/out/build/aoc-2023-5.avif");
        let mut response = request.into_ok_response()?;
        response.write_all(favicon_bytes)?;
        Ok(())
    })?;

    server.fn_handler("/aoc-2022-6.avif", Method::Get, |request| -> Result<(), EspIOError> {
        let favicon_bytes: &'static [u8] = include_bytes!("/IdeaProjects/advent-of-code-rust/advent-of-code-esp/target/xtensa-esp32s3-espidf/debug/build/site-206c1eb8fbc83c6e/out/build/aoc-2022-6.avif");
        let mut response = request.into_ok_response()?;
        response.write_all(favicon_bytes)?;
        Ok(())
    })?;

    server.fn_handler("/aoc-2023-4.avif", Method::Get, |request| -> Result<(), EspIOError> {
        let favicon_bytes: &'static [u8] = include_bytes!("/IdeaProjects/advent-of-code-rust/advent-of-code-esp/target/xtensa-esp32s3-espidf/debug/build/site-206c1eb8fbc83c6e/out/build/aoc-2023-4.avif");
        let mut response = request.into_ok_response()?;
        response.write_all(favicon_bytes)?;
        Ok(())
    })?;

    server.fn_handler("/aoc-2022.avif", Method::Get, |request| -> Result<(), EspIOError> {
        let favicon_bytes: &'static [u8] = include_bytes!("/IdeaProjects/advent-of-code-rust/advent-of-code-esp/target/xtensa-esp32s3-espidf/debug/build/site-206c1eb8fbc83c6e/out/build/aoc-2022.avif");
        let mut response = request.into_ok_response()?;
        response.write_all(favicon_bytes)?;
        Ok(())
    })?;

    server.fn_handler("/aoc-2023.avif", Method::Get, |request| -> Result<(), EspIOError> {
        let favicon_bytes: &'static [u8] = include_bytes!("/IdeaProjects/advent-of-code-rust/advent-of-code-esp/target/xtensa-esp32s3-espidf/debug/build/site-206c1eb8fbc83c6e/out/build/aoc-2023.avif");
        let mut response = request.into_ok_response()?;
        response.write_all(favicon_bytes)?;
        Ok(())
    })?;

    server.fn_handler("/aoc-2024.avif", Method::Get, |request| -> Result<(), EspIOError> {
        let favicon_bytes: &'static [u8] = include_bytes!("/IdeaProjects/advent-of-code-rust/advent-of-code-esp/target/xtensa-esp32s3-espidf/debug/build/site-206c1eb8fbc83c6e/out/build/aoc-2024.avif");
        let mut response = request.into_ok_response()?;
        response.write_all(favicon_bytes)?;
        Ok(())
    })?;

    server.fn_handler("/aoc-2023-1.avif", Method::Get, |request| -> Result<(), EspIOError> {
        let favicon_bytes: &'static [u8] = include_bytes!("/IdeaProjects/advent-of-code-rust/advent-of-code-esp/target/xtensa-esp32s3-espidf/debug/build/site-206c1eb8fbc83c6e/out/build/aoc-2023-1.avif");
        let mut response = request.into_ok_response()?;
        response.write_all(favicon_bytes)?;
        Ok(())
    })?;

    server.fn_handler("/aoc-2022-3.avif", Method::Get, |request| -> Result<(), EspIOError> {
        let favicon_bytes: &'static [u8] = include_bytes!("/IdeaProjects/advent-of-code-rust/advent-of-code-esp/target/xtensa-esp32s3-espidf/debug/build/site-206c1eb8fbc83c6e/out/build/aoc-2022-3.avif");
        let mut response = request.into_ok_response()?;
        response.write_all(favicon_bytes)?;
        Ok(())
    })?;

    server.fn_handler("/aoc-2023-2.avif", Method::Get, |request| -> Result<(), EspIOError> {
        let favicon_bytes: &'static [u8] = include_bytes!("/IdeaProjects/advent-of-code-rust/advent-of-code-esp/target/xtensa-esp32s3-espidf/debug/build/site-206c1eb8fbc83c6e/out/build/aoc-2023-2.avif");
        let mut response = request.into_ok_response()?;
        response.write_all(favicon_bytes)?;
        Ok(())
    })?;

    server.fn_handler("/aoc-2022-4.avif", Method::Get, |request| -> Result<(), EspIOError> {
        let favicon_bytes: &'static [u8] = include_bytes!("/IdeaProjects/advent-of-code-rust/advent-of-code-esp/target/xtensa-esp32s3-espidf/debug/build/site-206c1eb8fbc83c6e/out/build/aoc-2022-4.avif");
        let mut response = request.into_ok_response()?;
        response.write_all(favicon_bytes)?;
        Ok(())
    })?;

    server.fn_handler("/aoc-2022-2.avif", Method::Get, |request| -> Result<(), EspIOError> {
        let favicon_bytes: &'static [u8] = include_bytes!("/IdeaProjects/advent-of-code-rust/advent-of-code-esp/target/xtensa-esp32s3-espidf/debug/build/site-206c1eb8fbc83c6e/out/build/aoc-2022-2.avif");
        let mut response = request.into_ok_response()?;
        response.write_all(favicon_bytes)?;
        Ok(())
    })?;

    server.fn_handler("/aoc-2022-1.avif", Method::Get, |request| -> Result<(), EspIOError> {
        let favicon_bytes: &'static [u8] = include_bytes!("/IdeaProjects/advent-of-code-rust/advent-of-code-esp/target/xtensa-esp32s3-espidf/debug/build/site-206c1eb8fbc83c6e/out/build/aoc-2022-1.avif");
        let mut response = request.into_ok_response()?;
        response.write_all(favicon_bytes)?;
        Ok(())
    })?;

    server.fn_handler("/aoc-2023-3.avif", Method::Get, |request| -> Result<(), EspIOError> {
        let favicon_bytes: &'static [u8] = include_bytes!("/IdeaProjects/advent-of-code-rust/advent-of-code-esp/target/xtensa-esp32s3-espidf/debug/build/site-206c1eb8fbc83c6e/out/build/aoc-2023-3.avif");
        let mut response = request.into_ok_response()?;
        response.write_all(favicon_bytes)?;
        Ok(())
    })?;


    Ok(())
}