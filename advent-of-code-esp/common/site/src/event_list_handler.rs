use std::collections::BTreeMap;
use std::time::Instant;

use advent_of_code_solutions::aoc_solution::AoCSolution;
use anyhow::Result;
use askama::Template;
use embedded_svc::{http::Method, io::Write};
use esp_idf_svc::http::server::EspHttpServer;
use esp_idf_svc::io::EspIOError;
use esp_idf_sys::esp_get_minimum_free_heap_size;

use crate::multipart::parse_http_request;

#[derive(Template)]
#[template(path = "day-list.html")]
struct CalendarTemplate<'a> {
    current_year: u32,
    image_name: String,
    aoc: &'a BTreeMap<u32, Box<dyn AoCSolution>>,
}

#[derive(Template)]
#[template(path = "day.html")]
pub struct DayTemplate<'a> {
    current_year: u32,
    day: u32,
    image_name: String,
    sol: &'a Box<dyn AoCSolution>,
    complete: i32,
    runtime: String,
    free_heap_size_before: u32,
    free_heap_size_after: u32,
}

pub fn load_and_serve_event(
    server: &mut EspHttpServer,
    year: u32,
    aoc: &BTreeMap<u32, Box<dyn AoCSolution>>,
) -> Result<()> {
    let mut current_year = crate::globals::CURRENT_YEAR.lock();
    *current_year = year;

    //**********************************************************************************************
    // year pages
    //**********************************************************************************************
    let events = CalendarTemplate {
        current_year: year,
        image_name: format!("aoc-{}.avif", current_year),
        aoc: &aoc,
    };
    let events_string = events.render().unwrap().clone();
    server.fn_handler(format!("/{}", year).as_str(), Method::Get, move |request| -> Result<(), EspIOError> {
        let mut response = request.into_ok_response()?;
        response.write_all(events_string.as_bytes())?;
        Ok(())
    })?;

    //**********************************************************************************************
    // day pages get
    //**********************************************************************************************
    for (day, sol) in aoc {
        let event = DayTemplate {
            current_year: year,
            day: *day,
            image_name: format!("aoc-{}-{}.avif", year, day),
            sol: &sol,
            complete: 0,
            runtime: "".to_string(),
            free_heap_size_before: 0,
            free_heap_size_after: 0,
        };
        let event_string = event.render().unwrap().clone();
        server.fn_handler(
            format!("/{}/day/{}", year, day).as_str(),
            Method::Get,
            move |request| -> Result<(), EspIOError> {
                let mut response = request.into_ok_response()?;
                response.write_all(event_string.as_bytes())?;
                Ok(())
            },
        )?;
    }

    //**********************************************************************************************
    // day pages post
    //**********************************************************************************************
    for (day, sol) in aoc {
        let sol_clone = sol.factory();
        server.fn_handler(
            format!("/{}/day/{}", year, day).as_str(),
            Method::Post,
            move |mut request| -> Result<(), EspIOError> {
                let level;
                let puzzle_upload;
                let puzzle_answer;

                match parse_http_request(&mut request) {
                    Some((lev, puz_up, puz_ans)) => {
                        level = lev;
                        puzzle_upload = puz_up;
                        puzzle_answer = puz_ans;
                    }
                    None => {
                        request
                            .into_status_response(413)?
                            .write_all("Wrong request.".as_bytes())?;
                        return Ok(());
                    }
                }

                let complete: i32;
                let minimum_free_heap_size_before: u32;
                let minimum_free_heap_size_after: u32;
                let start = Instant::now();
                unsafe {
                    minimum_free_heap_size_before = esp_get_minimum_free_heap_size();
                }

                if level == "1" {
                    match puzzle_answer.parse::<String>() {
                        Ok(int_value) => {
                            if int_value == sol_clone.part_1_final(&puzzle_upload) {
                                complete = 1
                            } else {
                                complete = -1
                            }
                        }
                        Err(_) => {
                            request
                                .into_status_response(413)?
                                .write_all("Wrong request.".as_bytes())?;
                            return Ok(());
                        }
                    }
                } else if level == "2" {
                    match puzzle_answer.parse::<String>() {
                        Ok(int_value) => {
                            if int_value == sol_clone.part_2_final(&puzzle_upload) {
                                complete = 2
                            } else {
                                complete = -2
                            }
                        }
                        Err(_) => {
                            request
                                .into_status_response(413)?
                                .write_all("Wrong request.".as_bytes())?;
                            return Ok(());
                        }
                    }
                } else {
                    request
                        .into_status_response(413)?
                        .write_all("Wrong request.".as_bytes())?;
                    return Ok(());
                }

                let duration = start.elapsed();
                unsafe {
                    minimum_free_heap_size_after = esp_get_minimum_free_heap_size();
                }

                let event = DayTemplate {
                    current_year: year,
                    day: sol_clone.day(),
                    image_name: format!("aoc-{}-{}.avif", year, sol_clone.day()),
                    sol: &sol_clone,
                    complete: complete,
                    runtime: format!("{:?}", duration),
                    free_heap_size_before: minimum_free_heap_size_before,
                    free_heap_size_after: minimum_free_heap_size_after,
                };
                let event_string = event.render().unwrap().clone();

                let mut response = request.into_ok_response()?;
                response.write_all(event_string.as_bytes())?;
                Ok(())
            },
        )?;
    }

    Ok(())
}
