use reqwest::{self, Response};
use std;
use serde_json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct ResponseApi {
    text: String
}

struct MyError {
    
}

pub async fn get_response() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client.post("https://api.openai.com/v1/completions").send().await?;
    let res_text = res.text().await?;
    let res_api: ResponseApi = serde_json::from_reader(rdr);
    Ok(())
}

fn main() {


}
