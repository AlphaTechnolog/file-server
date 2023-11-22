use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
};

use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::env::get_base_path;
use crate::files::Files;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct ServeError(String);

fn parse_path(path: String) -> String {
    const CUSTOM_SEP: &'static str = "~";
    const SEP: &'static str = "/";
    path.replace(CUSTOM_SEP, SEP)
}

fn get_path(opt_path: &Option<Path<String>>) -> String {
    let mut path = get_base_path();

    if let Some(desired_path) = opt_path {
        let parsed_path = parse_path(desired_path.to_string());
        path.push_str(format!("/{}", parsed_path).as_str());
    }

    return path;
}

pub async fn read_file(opt_path: Option<Path<String>>) -> Response {
    let mut buffer = Vec::<u8>::new();

    let mut file = match File::open(get_path(&opt_path).as_str()) {
        Ok(opened_file) => opened_file,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ServeError(err.to_string())),
            )
                .into_response()
        }
    };

    if let Err(err) = file.read_to_end(&mut buffer) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ServeError(err.to_string())),
        )
            .into_response();
    }

    println!("before");

    buffer.into_response()
}

pub async fn serve_files(opt_path: Option<Path<String>>) -> Response {
    match Files::from_path(get_path(&opt_path)) {
        Ok(files) => Json(files.entries).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ServeError(err.to_string())),
        )
            .into_response(),
    }
}
