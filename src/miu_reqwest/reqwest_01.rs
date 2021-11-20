use reqwest;
use reqwest::{Error, Response};
use std::future::Future;
use serde_json::Value;
use std::collections::HashMap;
use serde::Deserialize;
use std::iter::Map;

#[derive(Deserialize, Debug)]
struct ResponseCode {
    code: u32,
    response: ResponseBody
}

#[derive(Deserialize, Debug)]
struct ResponseBody {
    author: String,
    id: u32,
    quote: String
}

pub async fn reqwest_miu(url: String) -> Result<String,Box<dyn std::error::Error> > {

    dbg!(String::from("Starting reqwest"));

    let client = reqwest::Client::builder()
        .build()?;

    let body = client.get(url)
        .send()
        .await?
        .json::<ResponseCode>()
        .await?;

    println!("Stuff here: {:?}", body.response.quote);

    Ok(String::from("All oke"))
}
