mod generator;
mod invoice;
#[macro_use]
extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}
use generator::init;
use invoice::{Invoice, Racun};
use rocket::serde::json::Json;

#[post("/invoice", format = "json", data = "<invoice>")]
fn new_invoice(invoice: Json<Racun>) -> &'static str {
    init(&invoice.0, None);
    "OK"
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, new_invoice])
}
