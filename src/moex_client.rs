use ureq::Error;
use ureq::Response;

mod domain;
use domain::MoexEgine;

const MOEX_ENGINES: &str = "https://iss.moex.com/iss/engines.json";

pub fn request_engines() -> Vec<MoexEgine> {
    let mut result = Vec::new();
    let request_result = send_request(MOEX_ENGINES.to_string());
    println!("{:#?}", request_result);
    let json_result: Result<serde_json::Value, serde_json::Error> =
        serde_json::from_str(&request_result);
    match json_result {
        Ok(v) => {
            let data = v["engines"]["data"].as_array().unwrap();
            let count = data.len();
            println!("Engines count {}", count);
            for engine in data {
                let moex_engine = domain::from_json_val(engine);
                result.push(moex_engine);
            }
        }
        Err(e) => println!("Error:{}", e),
    }
    println!("{:?}", result);
    return result;
}

fn send_request(url: String) -> String {
    let response = ureq::get(url.as_str()).call();
    return get_body(response);
}

fn get_body(response: Result<ureq::Response, ureq::Error>) -> String {
    return match response {
        Ok(rsp) => match rsp.into_string() {
            Ok(str) => str,
            Err(err_message) => err_message.to_string(),
        },
        Err(err_message) => err_message.to_string(),
    };
}
