use cucumber::gherkin::Scenario;
use hyper::{body, Client, Uri};
use serde_json::{Value};

use crate::World;

pub async fn before(scenario: &Scenario, w: &World) {
    println!("before hook");

    let script_file = check_contract_server_script_file(scenario);

    if script_file.len() > 0 {
        init_contract(&script_file, &w).await;
    }
}

pub async fn after(_scenario: &Scenario, ow: Option<&mut World>) {
    println!("after hook");

    let w = ow.unwrap();

    w.stop_pubnub();
    
    let contract_result = check_contract(&w).await;

    let expectations = get_json_value(contract_result, "expectations");
    let failed = get_json_value(expectations, "failed");
    
    if failed.as_array().unwrap().len() > 0 {
        println!("There are expect errors. The contract failed.");
        panic!("The step failed due to contract server expectations.");
    }

    println!("There are no expect errors. The contact succeeded.");
}

fn check_contract_server_script_file(scenario: &Scenario) -> String {
    for tag in scenario.tags.iter() {
        if tag.starts_with("contract") {
            return tag.replace("contract=", "");
        }
    }

    return String::from("");
}

async fn init_contract(script_file: &str, w: &World) {
    let contract_uri = format!(
        "http://{}/init?__contract__script__={}",
        w.settings.contract_server,
        script_file
    );

    println!("contract server init uri: {}", contract_uri);

    let client = Client::new();

    let response = client.get(contract_uri.parse::<Uri>().unwrap()).await;
    let contract_response = response.unwrap();
    let response_status = contract_response.status();
    let response_body = contract_response.into_body();
    
    let bytes = body::to_bytes(response_body).await;

    let json_string = String::from_utf8(bytes.unwrap().to_vec()).expect("response was not valid utf-8");
    let json: Value = serde_json::from_str(&json_string).unwrap();
    let script = get_json_value(json, "script");
    
    println!("contract server initialized with  response: {} {}", script_file, response_status);
    println!("script {}", script);  
}

async fn check_contract(w: &World) -> Value {
    let contract_uri = format!(
        "http://{}/expect",
        w.settings.contract_server,
    );

    println!("contract server expect uri: {}", contract_uri);

    let client = Client::new();

    let response = client.get(contract_uri.parse::<Uri>().unwrap()).await;
    let contract_response = response.unwrap();
    let response_status = contract_response.status();
    let response_body = contract_response.into_body();
    
    let bytes = body::to_bytes(response_body).await;

    let json_string = String::from_utf8(bytes.unwrap().to_vec()).expect("response was not valid utf-8");
    let json: Value = serde_json::from_str(&json_string).unwrap();
    
    println!("contract server expect result: {}", response_status);
    println!("expect: {}", json);

    json
}

fn get_json_value(json: Value, key: &str) -> Value {
    let value = json.as_object().unwrap().get(key).unwrap();
    value.clone()
}