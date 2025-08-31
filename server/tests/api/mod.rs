mod auth;
mod cook_and_run;

fn get_client() -> (reqwest::blocking::Client, String) {
    (
        reqwest::blocking::Client::new(),
        "http://0.0.0.0:3000".to_string(),
    )
}
