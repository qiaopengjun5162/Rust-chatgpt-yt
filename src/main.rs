use dotenv::dotenv;
use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use std::env;
use std::io::{stdin, stdout, Write};

// a struct to work with the API response
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct OAIResponse {
    id: Option<String>,
    object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Vec<OAIChoices>,
}

// a struct for the choices
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct OAIChoices {
    text: String,
    index: u8,
    logprobs: Option<u8>,
    finish_reason: String,
}

// a struct for the request you will make to the API
#[derive(Serialize, Debug)]
struct OAIRequest {
    prompt: String,
    max_tokens: u16,
}

// tokio async main function
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // load my env variables
    dotenv().ok();
    // create a Httpconnector, hyper
    let https = HttpsConnector::new();
    // create a client
    let client = Client::builder().build(https);
    // let client = reqwest::Client::builder().build()?;
    // URL to which we will make the request
    let uri = "https://api.openai.com/v1/engines/text-davinci-003/completions";
    // preamble, prompt to chatGPT
    let preamble = "Generate a Sql code for the given statement";
    // token, in the header
    let oai_token: String = env::var("OAI_TOKEN").unwrap();
    let auth_header_val = format!("Bearer {}", oai_token);
    println!("auth_header_val: {}", auth_header_val);
    // 打印出ASCII码为27的字符，即转义字符"ESC"
    println!("{esc}c", esc = 27 as char);

    loop {
        // Loop, inside the loop a way to read user input
        print!(">");
        stdout().flush().unwrap();
        let mut user_text = String::new();
        stdin()
            .read_line(&mut user_text)
            .expect("Failed to read user input");
        println!("");
        // spinner, wait for the response
        let mut sp = Spinner::new(Spinners::Dots12, "\t\tOpenAI is Thinking...".into());
        // request to chatGPT for every single user input, loop
        let oai_request = OAIRequest {
            prompt: format!("{preamble} {user_text}"),
            max_tokens: 1000,
        };

        let body = Body::from(serde_json::to_vec(&oai_request)?);

        println!("body: {:?}", body);
        let req = Request::post(uri)
            .header(header::CONTENT_TYPE, "application/json")
            .header("Authorization", &auth_header_val)
            .body(body)
            .expect("request failed");
        // reponse and we print that response
        let res = client.request(req).await?;
        let body = hyper::body::aggregate(res).await?;
        let json: OAIResponse = serde_json::from_reader(body.reader())?;

        // let mut headers = reqwest::header::HeaderMap::new();
        // headers.insert("Authorization", auth_header_val.parse()?);

        // headers.insert("Content-Type", "application/json".parse()?);
        // let user_text = "select all records from employee table";

        // let data = format!(
        //     r#"{{ "prompt": "{preamble} {user_text}", "max_tokens": 1000 }}"#,
        //     preamble = preamble,
        //     user_text = user_text
        // );
        // println!("data: {:?}", data);
        // let json: serde_json::Value = serde_json::from_str(&data)?;
        // let request = client
        //     .request(reqwest::Method::POST, uri)
        //     .headers(headers)
        //     .json(&json);
        // let response = request.send().await?;
        // let json: OAIResponse = response.json().await?;
        // println!("json: {:#?}", json);
        // let body = response.text().await?;
        // println!("body: {}", body);

        sp.stop();
        println!("");
        println!("{}", json.choices[0].text);
    }
    #[allow(unreachable_code)]
    Ok(())
}
// creat a table students with columns name,age,gender
// select all records from employee table
