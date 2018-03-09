#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate reqwest;

use rocket_contrib::{Json, Value};

fn root_req(endpoint: &str, body: Value) -> Value {
    let root = "https://sandbox.root.co.za/v1/insurance/".to_owned() + endpoint;
    
    let client = reqwest::Client::new();

    let mut res = client
        .post(root.as_str())
        .basic_auth("sandbox_MWY3MGM0Y2UtNDkwMS00MGM1LTkyMGMtN2U0NjRjYzE1MWEzLkY2VE12TVp6SVBSUmtrT3Z4ZTFqWldWNEJJTV9yTWdS", Some(""))
        .json(&body)
        .send()
        .unwrap();

    res.json().unwrap()
}

fn quote(params: &Value, context: String) -> Value {
    let model = &params["model"];

    let quotes: Value = root_req("quotes", json!({
        "type": "root_gadgets",
        "model_name": model
    }));


    let premium = &quotes[0]["suggested_premium"].as_i64().unwrap();
    let quote_package_id = &quotes[0]["quote_package_id"];

    let text = format!("As a courtesy to you, we'll add theft insurance as well! All for only R{}! Sound good?", premium / 100);

    let ctx = "projects/wineston-168d8/agent/sessions/4559c690-b0bd-64ae-dca1-963eba936475/contexts/winesure"

    json!({
        "fulfillmentText": text,
        "outputContexts": [{
            "name": context,
            "parameters": {
                "quote_package_id": quote_package_id
            }
        }]
    })
}

fn policyholder(params: &Value, context: String) -> Value {
    let serial = &params["serial"];
    let id = &params["id"];
    let fname = &params["fname"];
    let lname = &params["lname"];
    let email = &params["email"];

    let policyholder: Value = root_req("policyholders", json!({
        "id": {
            "type": "id",
            "number": id,
            "country": "ZA"
        },
        "first_name": fname,
        "last_name": lname,
        "email": email
    }));

    let policyholder_id = &policyholder["policyholder_id"];

    let text = format!("Thanks {}, are you sure you want to go through with this? Reply \"Trust me buddy\" to confirm, or \"Git good\" to cancel.", fname);

    json!({
        "fulfillmentText": text,
        "outputContexts": [{
            "name": context,
            "parameters": {
                "policyholder_id": policyholder_id
            }
        }]
    }) 
}

fn application(context: &Value) -> Value {
    let application: Value = root_req("applications", json!({
        "policyholder_id": context["policyholder_id"],
        "quote_package_id": context["quote_package_id"],
        "monthly_premium": context["monthly_premium"],
        "serial_number": context["serial_number"]
    }));

    // let application_id = 

    let policy: Value = root_req("policies", json!({
        "application_id": &application["application_id"]
    }));

    let policy_number = &policy["policy_number"];

    let text = format!("Thank you for being responsible, here is your new policy number: {}", policy_number);

    json!({
        "fulfillmentText": text
    })
}

#[post("/", data="<body>")]
fn webhook(body: Json<Value>) -> Json<Value> {
    let query_result = &body["queryResult"]; 
    let params = &query_result["parameters"];
    let action = query_result["action"].as_str().unwrap();

    let session = &body["session"];
    let context = session.to_string() + "/contexts/winesure";

    Json(match action {
        "quote" => quote(params, context),
        "user" => policyholder(params, context),
        "application" => {
            let outputContexts = query_result["oututContexts"].as_array().unwrap();
            let context = outputContexts.iter().find(|&c| c["name"] == context).unwrap();

            application(context)
        },
        _ => json!({ "fulfillmentText": "I'm confused..." })
    })
}

fn main() {
    rocket::ignite().mount("/", routes![webhook]).launch();
}
