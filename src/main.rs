#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate reqwest;

use rocket_contrib::{Json, Value};

const ROOT_URL: &'static str = "https://sandbox.root.co.za/v1/insurance/quotes";

fn quote(params: &Value) -> Value {
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

    json!({
        "fulfillmentText": text,
    })
}

#[post("/", data="<body>")]
fn webhook(body: Json<Value>) -> Json<Value> {
    let query_result = &body["queryResult"]; 
    let params = &query_result["parameters"];
    let action = query_result["action"].as_str().unwrap();

    Json(match action {
        "quote" => quote(params),
        _ => json!({ "fulfillmentText": "I'm confused..." })
    })
}

fn main() {
    rocket::ignite().mount("/", routes![webhook]).launch();
}
