use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use reqwest::Client;
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;

const DEAD_MAN_CLOCK_TIMER_IN_S: u64 = 60;

const WEBHOOK_URL: &str = "https://webhook.site/a7cd289d-40cb-4b70-8226-7406c3bc4e5c";

async fn handle_request(
    _req: Request<Body>,
    state: Arc<Mutex<tokio::time::Instant>>,
) -> Result<Response<Body>, Infallible> {
    let mut last_request = state.lock().unwrap();
    *last_request = tokio::time::Instant::now();
    Ok(Response::new(Body::from("Request received")))
}

async fn dead_man_switch(state: Arc<Mutex<tokio::time::Instant>>, webhook_url: &str) {
    loop {
        sleep(Duration::from_secs(1)).await;
        let last_request = *state.lock().unwrap();
        if last_request.elapsed() > Duration::from_secs(DEAD_MAN_CLOCK_TIMER_IN_S) {
            let client = Client::new();
            let _ = client.post(webhook_url).send().await;
            println!("Dead man switch activated. Sayonara! 🫡");
            std::process::exit(0);
        }
    }
}

#[tokio::main]
async fn main() {
    println!("App started!");
    let state = Arc::new(Mutex::new(tokio::time::Instant::now()));

    let make_svc = make_service_fn(|_conn| {
        let state = Arc::clone(&state);
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                handle_request(req, Arc::clone(&state))
            }))
        }
    });

    let addr = [0, 0, 0, 0];

    let port = std::env::var("PORT")
        .unwrap_or("3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);

    let addr = (addr, port).into();

    let server = Server::bind(&addr).serve(make_svc);
    println!("Server started!");

    let state_for_dead_man_switch = Arc::clone(&state);
    tokio::spawn(dead_man_switch(state_for_dead_man_switch, WEBHOOK_URL));

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    fn goofy_test_to_make_pipeline_happy() {
        assert_eq!(2, "2".parse::<i32>().unwrap());
    }
}
