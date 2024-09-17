use actix_web::{web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use log::info;
use serde::{Deserialize, Serialize};

/// Represents a date range with start and end times in UTC.
#[derive(Debug, Serialize, Deserialize)]
struct DateRange {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

type DateRangeJSON = web::Json<DateRange>;

/// Handles POST requests to get UTC date range.
/// Logs the received date range and returns a success message.
/// Note: From the frontend, send `new Date().toISOString()` for date fields.
#[actix_web::post("/")]
pub async fn get_utc(date_range: DateRangeJSON) -> impl Responder {
    info!("Received date range: {:?}", date_range);
    HttpResponse::Ok().body("Request processed successfully!")
}
