use embedded_svc::http::Headers;
use embedded_svc::http::server::Request;
use embedded_svc::io::Read;
use esp_idf_svc::http::server::EspHttpConnection;

const MAX_LEN: usize = 1024 * 1024;

fn extract_boundary(content_type: &str) -> Option<String> {
    if !content_type.starts_with("multipart/form-data") {
        return None;
    }

    let parts: Vec<&str> = content_type.split(';').collect();
    for part in parts {
        if part.trim().starts_with("boundary=") {
            return Some(part.trim().replace("boundary=", ""));
        }
    }
    None
}

fn parse_multipart_form_data(input: &Vec<u8>, boundary: &str) -> Option<(String, String, String)> {
    let boundary = format!("--{}", boundary);
    let input_str = String::from_utf8_lossy(input);

    // Teile den Input in Abschnitte basierend auf dem Boundary
    let parts: Vec<&str> = input_str.split(&boundary).collect();

    let mut level = String::new();
    let mut puzzle_upload = String::new();
    let mut puzzle_answer = String::new();

    for part in parts {
        if part.contains("Content-Disposition: form-data; name=\"level\"") {
            level = part
                .split("\r\n\r\n")
                .nth(1)
                .unwrap_or_default()
                .trim()
                .to_string();
        } else if part.contains("Content-Disposition: form-data; name=\"puzzle-upload-1\"") {
            puzzle_upload = part
                .split("\r\n\r\n")
                .nth(1)
                .unwrap_or_default()
                .trim()
                .to_string();
        } else if part.contains("Content-Disposition: form-data; name=\"puzzle-answer-1\"") {
            puzzle_answer = part
                .split("\r\n\r\n")
                .nth(1)
                .unwrap_or_default()
                .trim()
                .to_string();
        } else if part.contains("Content-Disposition: form-data; name=\"puzzle-upload-2\"") {
            puzzle_upload = part
                .split("\r\n\r\n")
                .nth(1)
                .unwrap_or_default()
                .trim()
                .to_string();
        } else if part.contains("Content-Disposition: form-data; name=\"puzzle-answer-2\"") {
            puzzle_answer = part
                .split("\r\n\r\n")
                .nth(1)
                .unwrap_or_default()
                .trim()
                .to_string();
        }
    }

    if level.is_empty() || puzzle_upload.is_empty() || puzzle_answer.is_empty() {
        None
    } else {
        Some((level, puzzle_upload, puzzle_answer))
    }
}

pub fn parse_http_request(
    request: &mut Request<&mut EspHttpConnection>,
) -> Option<(String, String, String)> {
    let len = request.content_len().unwrap() as usize;
    if len > MAX_LEN {
        return None;
    }

    let mut buf = vec![0; len];
    let _ = request.read_exact(&mut buf);
    let content_type = request.content_type().map(|ct| ct.to_owned());

    let mut boundary = String::new();
    if let Some(content_type) = content_type {
        match extract_boundary(&content_type) {
            Some(res) => boundary = res,
            None => {
                return None;
            }
        }
    }

    let mut _level = String::new();
    let mut _puzzle_upload = String::new();
    let mut _puzzle_answer = String::new();
    match parse_multipart_form_data(&buf, &boundary) {
        Some((lev, puz_up, puz_ans)) => {
            _level = lev;
            _puzzle_upload = puz_up;
            _puzzle_answer = puz_ans;
        }
        None => {
            return None;
        }
    }

    if _level.is_empty() || _puzzle_upload.is_empty() || _puzzle_answer.is_empty() {
        None
    } else {
        Some((_level, _puzzle_upload, _puzzle_answer))
    }
}
