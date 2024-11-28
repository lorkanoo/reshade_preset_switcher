use reqwest::Error;

pub mod gw2;

fn get_sync(url: String) -> Result<reqwest::blocking::Response, Error> {
    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_hostnames(true)
        .use_rustls_tls()
        .build()
        .expect("error");
    client.get(url).send()
}
