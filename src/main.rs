use std::{thread::sleep, time::Duration};

use serde_json::Value;
use ureq::Response;
use ureq::Error;

mod moex_client;
use moex_client::request_engines;

// Example URL "http://iss.moex.com/iss/history/engines/stock/markets/shares/boards/tqbr/securities.json?date=2021-04-01";
const MOEX_ENGINES: &str = "https://iss.moex.com/iss/engines.json";
const MOEX_BY_DATE_URL: &str = "http://iss.moex.com/iss/history/engines/stock/markets/shares/boards/tqbr/securities.json?date=";

fn main() -> Result<(), ureq::Error>{
    println!("MOEX client start...");
    println!("Request MOEX engines");
    request_engines();
    sleep(Duration::from_secs(1));

    let mut i = 1;
    loop {
        let url = get_request_url(4, i);
        println!("Send request to url:{}", url);
        sleep(Duration::from_secs(1));
        let request_result = send_request(url);
        println!("{:#?}", request_result);
        if i > 30 {
            break;
        }
        i = i + 1;
        sleep(Duration::from_secs(1))
    }

    println!("MOEX client stop");
    Ok(())
}

fn send_request(url: String) -> String {
    let response = ureq::get(url.as_str()).call();
    return get_body(response);
}

fn get_body(response: Result<Response, Error>) -> String{
    return match response {
        Ok(rsp) => {
            match rsp.into_string(){
                Ok(str) => str,
                Err(err_message) => err_message.to_string()
            }
        }
        Err(err_message) => err_message.to_string()
    };
}

fn get_request_url(month:u8, day:u8) -> String{
    let mut result: String = MOEX_BY_DATE_URL.to_owned();
    let request_date = get_request_date(month, day);
    result.push_str(&request_date);
    return result;
}

fn get_request_date(month:u8, day:u8) -> String{
    let mut result: String = "2021-".to_string();
    let month_str: String = get_request_month(month);
    let day_str: String = get_request_day(day);
    result.push_str(&month_str);
    result.push_str("-");
    result.push_str(&day_str);
    return result;
}

fn get_request_day(day:u8) -> String{
    let day_str: String = day.to_string();
    return left_pad(day_str);
}

fn get_request_month(month:u8) -> String{
    let month_str: String = month.to_string();
    return left_pad(month_str);
}

fn left_pad(src: String) -> String {
    return format!("{:0>2}", src);
}

/*fn request_engines() -> Vec<MoexEgine>{
    let mut result = Vec::new();
    let request_result = send_request(MOEX_ENGINES.to_string());
    println!("{:#?}", request_result);
    let json_result: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(&request_result);
    match json_result{
        Ok(v) => {
            let data = v["engines"]["data"].as_array().unwrap();
            let count = data.len();
            println!("Engines count {}", count);
            for engine in data {
                let moex_engine = domain::from_json_val(engine);
                result.push(moex_engine);
            }
        },
        Err(e) => println!("Error:{}", e)
    }
    println!("{:?}", result);
    return result;
}*/

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_left_pad() {
        assert_eq!(left_pad("2".to_string()), "02");
    }
    
    #[test]
    fn test_get_request_month() {
        assert_eq!(get_request_month(2), "02");
        assert_eq!(get_request_month(12), "12");
    }
    
    #[test]    
    fn test_get_request_day() {
        assert_eq!(get_request_day(2), "02");
        assert_eq!(get_request_day(22), "22");
    }
       
    #[test]    
    fn test_get_request_date() {
        assert_eq!(get_request_date(2, 3), "2021-02-03");
        assert_eq!(get_request_date(11, 22), "2021-11-22");
    }

           
    #[test]    
    fn test_get_request_url() {
        assert_eq!(get_request_url(2, 3),
            "http://iss.moex.com/iss/history/engines/stock/markets/shares/boards/tqbr/securities.json?date=2021-02-03");
        assert_eq!(get_request_url(11, 22),
            "http://iss.moex.com/iss/history/engines/stock/markets/shares/boards/tqbr/securities.json?date=2021-11-22");
    }

}