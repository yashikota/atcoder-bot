use reqwest::Error;
use worker::*;

mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

fn fetch_contest_info() -> Result<serde_json::Value, Error> {
    let url = "https://abc-latest.deno.dev/";
    let res = reqwest::get(url)?.text()?;
    let json: serde_json::Value = serde_json::from_str(&res).unwrap();
    Ok(json)
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    utils::set_panic_hook();

    let router = Router::new();

    router
        .get("/", |_, _| {
            let res = fetch_contest_info().unwrap();
            Response::ok(res)
        })
        .run(req, env)
        .await
}
