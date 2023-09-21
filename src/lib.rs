use rand::Rng;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::error::Error;

fn get_random_id() -> i32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(0..1000000000);
}

#[no_mangle]
pub fn translate(
    text: &str,
    from: &str,
    to: &str,
    _needs: HashMap<String, String>,
) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::blocking::ClientBuilder::new().build()?;
    const URL: &str = "https://interpreter.cyapi.cn/v1/translator";

    let body = json!({
        "source": text,
        "detect": true,
        "os_type": "ios",
        "device_id": "F1F902F7-1780-4C88-848D-71F35D88A602",
        "trans_type": format!("{from}2{to}"),
        "media": "text",
        "request_id": get_random_id(),
        "user_id": "",
        "dict": true
    });
    fn parse_result(res: &Value) -> Option<String> {
        let result = res.as_object()?.get("target")?.as_str()?.to_string();

        Some(result)
    }

    let res: Value = client
        .post(URL)
        .json(&body)
        .header("Content-Type", "application/json")
        .header("x-authorization", "token ssdj273ksdiwi923bsd9")
        .header(
            "user-agent",
            "caiyunInterpreter/5 CFNetwork/1404.0.5 Darwin/22.3.0",
        )
        .send()?
        .json()?;

    if let Some(result) = parse_result(&res) {
        return Ok(Value::String(result));
    } else {
        return Err("Response Parse Error".into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_request() {
        let needs = HashMap::new();
        let result = translate("hello", "auto", "zh", needs).unwrap();
        println!("{result}");
    }
}
