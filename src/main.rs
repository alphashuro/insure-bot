#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate reqwest;

use rocket_contrib::{Json, Value};

const ROOT_URL: &'static str = "https://sandbox.root.co.za/v1/insurance/quotes";

#[post("/", data="<body>")]
fn webhook(body: Json<Value>) -> Json<Value> {
    let params = &body["queryResult"]["parameters"];
    let model = &params["model"];

    let client = reqwest::Client::new();
    let mut res = client
        .post(ROOT_URL)
        .basic_auth("sandbox_MWY3MGM0Y2UtNDkwMS00MGM1LTkyMGMtN2U0NjRjYzE1MWEzLkY2VE12TVp6SVBSUmtrT3Z4ZTFqWldWNEJJTV9yTWdS", Some(""))
        .json(&json!({
            "type": "root_gadgets",
            "model_name": model
        }))
        .send()
        .unwrap();

    let quotes: Value = res.json().unwrap();
    let premium = quotes[0]["suggested_premium"].as_i64().unwrap();

    let text = format!("As a courtesy to you, we'll add theft insurance as well! All for only R{}! Sound good?", premium / 100);

    Json(json!({
        "fulfillmentText": text,
    }))
}

fn main() {
    rocket::ignite().mount("/", routes![webhook]).launch();
}
