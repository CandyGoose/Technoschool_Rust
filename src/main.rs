use axum::handler::Handler;
use axum::routing::{get, post};
use axum::{
    extract::{Extension, Form, Query},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Json},
    Router,
};
use chrono::{Datelike, Duration, NaiveDate};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tower::ServiceBuilder;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Event {
    id: u32,
    user_id: u32,
    date: NaiveDate,
    description: String,
}

#[derive(Debug)]
enum BusinessLogicError {
    EventNotFound,
    InvalidInput(String),
    Other(String),
}

type EventsDb = Arc<Mutex<HashMap<u32, Event>>>;

fn serialize_event(event: &Event) -> serde_json::Value {
    json!({
        "id": event.id,
        "user_id": event.user_id,
        "date": event.date.to_string(),
        "description": event.description,
    })
}

#[derive(Debug, Deserialize)]
struct CreateEventParams {
    user_id: u32,
    date: String,
    description: String,
}

impl CreateEventParams {
    fn validate(self) -> Result<Event, BusinessLogicError> {
        let date = NaiveDate::parse_from_str(&self.date, "%Y-%m-%d")
            .map_err(|_| BusinessLogicError::InvalidInput("Invalid date format".into()))?;
        Ok(Event {
            id: 0,
            user_id: self.user_id,
            date,
            description: self.description,
        })
    }
}

#[derive(Debug, Deserialize)]
struct UpdateEventParams {
    id: u32,
    user_id: u32,
    date: String,
    description: String,
}

impl UpdateEventParams {
    fn validate(self) -> Result<Event, BusinessLogicError> {
        let date = NaiveDate::parse_from_str(&self.date, "%Y-%m-%d")
            .map_err(|_| BusinessLogicError::InvalidInput("Invalid date format".into()))?;
        Ok(Event {
            id: self.id,
            user_id: self.user_id,
            date,
            description: self.description,
        })
    }
}

fn create_event(db: EventsDb, event: Event) -> Result<String, BusinessLogicError> {
    let mut db = db.lock().unwrap();
    let new_id = db.len() as u32 + 1;
    let mut new_event = event.clone();
    new_event.id = new_id;
    db.insert(new_id, new_event);
    Ok("Event created".into())
}

fn update_event(db: EventsDb, event: Event) -> Result<String, BusinessLogicError> {
    let mut db = db.lock().unwrap();
    if db.contains_key(&event.id) {
        db.insert(event.id, event);
        Ok("Event updated".into())
    } else {
        Err(BusinessLogicError::EventNotFound)
    }
}

fn delete_event(db: EventsDb, params: HashMap<String, String>) -> Result<String, BusinessLogicError> {
    let id = params.get("id")
        .ok_or_else(|| BusinessLogicError::InvalidInput("Missing id".into()))?
        .parse::<u32>()
        .map_err(|_| BusinessLogicError::InvalidInput("Invalid id".into()))?;

    let mut db = db.lock().unwrap();
    if db.remove(&id).is_some() {
        Ok("Event deleted".into())
    } else {
        Err(BusinessLogicError::EventNotFound)
    }
}

fn events_for_period(db: EventsDb, user_id: u32, start_date: NaiveDate, end_date: NaiveDate) -> Vec<Event> {
    let db = db.lock().unwrap();
    db.values()
        .filter(|event| {
            event.user_id == user_id && event.date >= start_date && event.date <= end_date
        })
        .cloned()
        .collect()
}

async fn create_event_handler(
    Extension(db): Extension<EventsDb>,
    Form(params): Form<CreateEventParams>,
) -> impl IntoResponse {
    match params.validate() {
        Ok(event) => match create_event(db, event) {
            Ok(msg) => Json(json!({ "result": msg })).into_response(),
            Err(e) => handle_business_error(e).into_response(),
        },
        Err(e) => handle_business_error(e).into_response(),
    }
}

async fn update_event_handler(
    Extension(db): Extension<EventsDb>,
    Form(params): Form<UpdateEventParams>,
) -> impl IntoResponse {
    match params.validate() {
        Ok(event) => match update_event(db, event) {
            Ok(msg) => Json(json!({ "result": msg })).into_response(),
            Err(e) => handle_business_error(e).into_response(),
        },
        Err(e) => handle_business_error(e).into_response(),
    }
}


async fn delete_event_handler(
    Extension(db): Extension<EventsDb>,
    Form(params): Form<HashMap<String, String>>,
) -> impl IntoResponse {
    match delete_event(db, params) {
        Ok(msg) => Json(json!({ "result": msg })).into_response(),
        Err(e) => handle_business_error(e).into_response(),
    }
}


#[derive(Debug, Deserialize)]
struct QueryParams {
    user_id: u32,
    date: String,
}

async fn events_for_day_handler(
    Extension(db): Extension<EventsDb>,
    Query(params): Query<QueryParams>,
) -> impl IntoResponse {
    match NaiveDate::parse_from_str(&params.date, "%Y-%m-%d") {
        Ok(date) => {
            let events = events_for_period(db, params.user_id, date, date);
            Json(json!({ "result": events })).into_response()
        }
        Err(_) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Invalid date format" })),
        )
            .into_response(),
    }
}

async fn events_for_week_handler(
    Extension(db): Extension<EventsDb>,
    Query(params): Query<QueryParams>,
) -> impl IntoResponse {
    match NaiveDate::parse_from_str(&params.date, "%Y-%m-%d") {
        Ok(date) => {
            let start_date = date;
            let end_date = date + Duration::days(7);
            let events = events_for_period(db, params.user_id, start_date, end_date);
            Json(json!({ "result": events })).into_response()
        }
        Err(_) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Invalid date format" })),
        )
            .into_response(),
    }
}

async fn events_for_month_handler(
    Extension(db): Extension<EventsDb>,
    Query(params): Query<QueryParams>,
) -> impl IntoResponse {
    match NaiveDate::parse_from_str(&params.date, "%Y-%m-%d") {
        Ok(start_date) => {
            let end_date = start_date + Duration::days(30);
            let events = events_for_period(db, params.user_id, start_date, end_date);
            Json(json!({ "result": events })).into_response()
        }
        Err(_) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Invalid date format" })),
        )
            .into_response(),
    }
}




fn handle_business_error(error: BusinessLogicError) -> impl IntoResponse {
    match error {
        BusinessLogicError::InvalidInput(msg) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": msg })),
        )
            .into_response(),
        BusinessLogicError::EventNotFound => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "error": "Event not found" })),
        )
            .into_response(),
        BusinessLogicError::Other(msg) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": msg })),
        )
            .into_response(),
    }
}

async fn logging_middleware<B>(
    req: http::Request<B>,
    next: Next<B>,
) -> impl IntoResponse {
    info!("Handling request: {} {}", req.method(), req.uri());
    let response = next.run(req).await;
    info!("Finished handling request");
    response
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

    let db: EventsDb = Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
        .route("/create_event", post(create_event_handler))
        .route("/update_event", post(update_event_handler))
        .route("/delete_event", post(delete_event_handler))
        .route("/events_for_day", get(events_for_day_handler))
        .route("/events_for_week", get(events_for_week_handler))
        .route("/events_for_month", get(events_for_month_handler))
        .layer(
            ServiceBuilder::new()
                .layer(Extension(db))
                .layer(axum::middleware::from_fn(logging_middleware)),
        );

    let port = 8080;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    info!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
