#[macro_use] extern crate rocket;
use rocket::http::Status;

#[get("/")]
fn healthz() -> Status {
    Status::Ok
}

#[get("/")]
fn handler() -> &'static str {
    "hello world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/healthz", routes![healthz])
        .mount("/", routes![handler])
}