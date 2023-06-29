mod generator;
mod invoice;
use std::thread;
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
    thread::spawn(move || {
        let _ = init(&invoice.0, None);
    });
    "OK"
}
#[get("/invoice/<id>")]
fn get_invoice(id: String) -> Json<Racun> {
    //Get the invoice freom the invoice folder
    // TODO: Just make some checks here so the program doesn't crash
    let output_file = format!("invoices\\{}\\output.json", id);
    //Get current directory path and append the invoice folder path
    let invoice_folder_path = std::env::current_dir().unwrap().join(output_file);
    //Read the file
    println!("Invoice path: {:?}", invoice_folder_path);
    let invoice_file = std::fs::read_to_string(invoice_folder_path).unwrap();
    //Parse the file
    let racun: Racun = serde_json::from_str(&invoice_file).unwrap();
    Json(racun)
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, new_invoice, get_invoice])
}
