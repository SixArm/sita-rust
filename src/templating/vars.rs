//! Vars.
//!
//! Variables as a struct for use with Tera templates.
//! We prefer this approach over a Tera Context object.

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Vars {
    pub title: Option<String>,
    pub content: Option<String>, // content as HTML text
}
