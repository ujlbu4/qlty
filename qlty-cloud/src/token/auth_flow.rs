use super::DEFAULT_USER;
use crate::token::{Token, SERVICE};
use actix_web::{
    dev::{Server, ServerHandle},
    get,
    http::header::LOCATION,
    rt,
    web::{Data, Query},
    App, HttpResponse, HttpResponseBuilder, HttpServer,
};
use anyhow::{Ok, Result};
use console::style;
use serde::Deserialize;
use std::{
    net::TcpListener,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use tracing::{debug, info, warn};
use uuid::Uuid;

const LOGIN_URL: &str = "https://qlty.sh/login";
const LOGIN_URL_ENV: &str = "QLTY_AUTH_LOGIN_URL";

#[derive(Deserialize, Debug)]
struct AuthFlowQueryParams {
    state: String,
    code: String,
    redirect_uri: String,
}

#[derive(Debug, Clone)]
pub struct AppState {
    original_state: String,
    server_handle: Arc<Mutex<Option<ServerHandle>>>,
    credential_user: String,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            original_state: Uuid::new_v4().to_string(),
            server_handle: Arc::new(Mutex::new(None)),
            credential_user: DEFAULT_USER.to_string(),
        }
    }
}

#[get("/")]
async fn auth_flow(
    query: Query<AuthFlowQueryParams>,
    state: Data<Arc<AppState>>,
) -> HttpResponseBuilder {
    if state.original_state != query.state {
        warn!(
            "Invalid state parameter in auth flow: {:?} did not match received state {:?}",
            state.original_state, query.state
        );
        HttpResponse::BadRequest()
    } else {
        let token = Token::new(&state.credential_user);

        match token.set(&query.code) {
            Result::Ok(_) => {
                debug!(
                    "Auth token stored in credential storage: {}.{}",
                    SERVICE, state.credential_user
                );
                stop_login_server(state);
                HttpResponse::TemporaryRedirect()
                    .insert_header((LOCATION, query.redirect_uri.clone()))
                    .take()
            }
            Err(e) => {
                warn!("Failed to store auth token in credential storage: {}", e);
                HttpResponse::BadRequest()
            }
        }
    }
}

fn login_server(state: Arc<AppState>) -> Result<(Server, u16)> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .service(auth_flow)
    })
    .shutdown_timeout(0)
    .workers(1)
    .listen(listener)?
    .run();
    Ok((server, port))
}

fn launch_login_server(state: AppState) -> Result<u16> {
    let handle = state.server_handle.clone();
    let (server, port) = login_server(Arc::new(state))?;

    handle.lock().unwrap().replace(server.handle());

    thread::spawn(move || {
        let _ = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(server);
    });

    Ok(port)
}

fn stop_login_server(state: Data<Arc<AppState>>) {
    rt::spawn(async move {
        tokio::time::sleep(Duration::from_millis(100)).await;
        let handle = state.server_handle.lock().unwrap().take().unwrap();
        handle.stop(true).await;
    });
}

pub fn auth_via_browser() -> Result<String> {
    let url = std::env::var(LOGIN_URL_ENV).unwrap_or_else(|_| LOGIN_URL.to_string());

    let state = AppState::default();
    let user = &state.credential_user;
    let original_state = &state.original_state;
    let port = launch_login_server(state.clone())?;
    let local_url = format!("http://localhost:{}", port);
    info!("Auth login server started on port {}", local_url);

    eprintln!(
        "Launching {} in your browser. Once you've logged in, come back to the terminal.",
        style("http://qlty.sh/login").bold().green()
    );
    thread::sleep(Duration::from_millis(500));

    let open_url = ureq::get(&url)
        .query("state", original_state)
        .query("response_type", "token")
        .query("redirect_uri", &local_url)
        .request_url()?
        .as_url()
        .to_string();
    info!("Opening browser to {}", open_url);
    webbrowser::open(&open_url)?;

    let token = Token::new(user);
    loop {
        if let Result::Ok(value) = token.get() {
            eprintln!("Login successful! Your credentials have been stored for future use.");
            return Ok(value);
        }
        thread::sleep(Duration::from_secs(1));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        test::{self, init_service, TestRequest},
        App,
    };

    impl AppState {
        fn test(original_state: &str) -> Self {
            let mut state = AppState::default();
            state.original_state = original_state.to_string();
            state.credential_user = "qlty-cli-test".to_string();
            state
        }
    }

    #[actix_web::test]
    async fn test_auth_flow_get() {
        let state = Arc::new(AppState::test("123"));
        let req = TestRequest::default()
            .uri("/?state=123&code=ABCDEFG&redirect_uri=https://example.com")
            .to_request();
        let app = init_service(App::new().app_data(Data::new(state)).service(auth_flow)).await;
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 307);
        assert_eq!(resp.headers().get(LOCATION).unwrap(), "https://example.com");
    }

    #[actix_web::test]
    async fn test_auth_flow_wrong_state() {
        let state = Arc::new(AppState::test("123"));
        let req = TestRequest::default()
            .uri("/?state=456&code=ABCDEFG&redirect_uri=https://example.com")
            .to_request();
        let app = init_service(App::new().app_data(Data::new(state)).service(auth_flow)).await;
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400);
    }

    #[actix_web::test]
    async fn test_auth_flow_no_redirect_uri() {
        let state = Arc::new(AppState::test("123"));
        let req = TestRequest::default()
            .uri("/?code=ABCDEFG&redirect_uri=https://example.com")
            .to_request();
        let app = init_service(App::new().app_data(Data::new(state)).service(auth_flow)).await;
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400);
    }

    #[actix_web::test]
    async fn test_auth_flow_server() {
        let state = AppState::test("123");
        let port = launch_login_server(state).unwrap();
        let url = format!("http://127.0.0.1:{}", port);

        ureq::get(&url)
            .query("state", "123")
            .query("code", "ABCDEFG")
            .query("redirect_uri", "https://example.com")
            .call()
            .unwrap();

        thread::sleep(Duration::from_millis(100));

        // ensure server has shut-down
        assert!(ureq::get(&url)
            .timeout(Duration::from_millis(10))
            .call()
            .is_err());

        let token = Token::new("qlty-cli-test").get().unwrap();
        assert_eq!(token, "ABCDEFG");
    }
}
