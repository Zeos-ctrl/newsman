use std::convert::Infallible;
use serde::Deserialize;
use warp::{http::StatusCode, Filter, self, body::content_length_limit};
use log::debug;

use crate::emails::{add_email, remove_email};

#[derive(Deserialize, Clone)]
pub struct Email {
    pub email: String,
}

pub async fn handle_email_post(email: Email) -> Result<impl warp::Reply, Infallible> {
    debug!("handling email post request...");
    match add_email(email.email).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Ok(StatusCode::BAD_REQUEST)
    }
}

pub fn remove_email_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("api" / "remove" / String)
        .and(warp::get())
        .and_then(handle_remove_email_get)
}

pub async fn handle_remove_email_get(token: String) -> Result<impl warp::Reply, Infallible> {
    debug!("handling email remove request...");
    match remove_email(token).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Ok(StatusCode::BAD_REQUEST)
    }
}

pub fn add_email_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    debug!("constructing route...");
    warp::path!("api" / "add")
        .and(warp::post())
        .and(content_length_limit(1024 * 16))
        .and(warp::body::form())
        .and_then(handle_email_post)
}

pub fn construct_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    add_email_route()
        .or(remove_email_route())
}
