// TODO:
// - Write better code
// - Move everything to its own file
// - Support multiple lights 
// - Autodiscover lights (mDNS?) 
// - Control brighness and temperature 
// - Handle parameters
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LightCollection {
    number_of_lights: u32,
    lights: Vec<Light>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Light {
    on: u8,
    brightness: u32,
    temperature: u32,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    println!("Making lights go on or off.");

    // make a local GET requesto to a single lights
    let url = "http://10.0.1.34:9123/elgato/lights";
    let body = reqwest::get(url)
        .await?
        .text()
        .await?;

    println!("Raw body:");
    println!("{:?}", body);

    let mut l: LightCollection = serde_json::from_str(body.as_str()).unwrap();

    // FIX: This is wildly unoptimal
    if l.lights[0].on == 0 {
        println!("The light is off");
        l.lights[0].on = 1;

        let client = reqwest::Client::new();
        let _toggle = client.put(url)
            .json(&l)
            .send()
            .await?;

    } else {
        println!("The light is on");
        l.lights[0].on = 0;

        let client = reqwest::Client::new();
        let _toggle = client.put(url)
            .json(&l)
            .send()
            .await?;
    }

    Ok(())
}
