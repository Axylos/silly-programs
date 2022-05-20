use std::io;
use axum::response::{IntoResponse, Response, Html};
use axum::{Json, Router};
use axum::routing::{get, get_service};
use askama::Template;
use axum::http::StatusCode;

#[derive(Template)]
#[template(path = "welcome.html")]
struct WelcomeTmpl {
   _name: String
}

struct HtmlTemplate<T>(T);

impl <T> IntoResponse for HtmlTemplate<T>
where T: Template, {
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template {}", err))
                    .into_response(),
            }
        }
    }
pub fn get_app() -> Router {
    println!("current path: {:?}", std::env::current_dir());
    let serve_dir = tower_http::services::ServeDir::new("static/");
    let serve_file = tower_http::services::ServeFile::new("./static/style.css");
    let app = Router::new()
        .route("/ping", get(ping))
        .route("/silly/*poing", get(ping))
        .route("/welcome", get(welcome))
        .route("/foo/*key", get_service(serve_file)
            .handle_error(|error: io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error)
                )
            }))
        .route("/static/*key", get_service(serve_dir)
            .handle_error(|error: io::Error| async move {
                eprintln!("can't find route");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error)
                    )
            }));


    app
}

async fn welcome() -> impl IntoResponse {
    let tmpl = WelcomeTmpl { _name: "hey there".to_string() };
    HtmlTemplate(tmpl)
}

async fn ping() -> impl IntoResponse {
   Json("pong")
}

#[cfg(test)]
mod tests {
    use std::net::{SocketAddr, TcpListener};
    use tower::ServiceExt;
    use scraper::{Html, Selector};
    use axum::{
        body::Body,
        http::{Request, StatusCode, header}
    };
    use super::*;
    #[tokio::test]
    async fn it_pings() {
        let app = get_app();

        let resp = app
            .oneshot(Request::builder().uri("/ping").body(Body::empty()).unwrap())
            .await
            .unwrap();


        let (parts, body) = resp.into_parts();
        let body = hyper::body::to_bytes(body).await.unwrap();
        let content_type = parts.headers.get(header::CONTENT_TYPE).unwrap();

        let data = String::from_utf8(body.to_vec()).unwrap();
        let pong = serde_json::json!("pong");
        assert_eq!(parts.status, StatusCode::OK);
        assert_eq!(data, pong.to_string());
        assert_eq!(mime::APPLICATION_JSON.as_ref(), content_type);
    }

    #[tokio::test]
    async fn test_welcome() {
        let app = get_app();

        let resp = app
            .oneshot(Request::builder().uri("/welcome").body(Body::empty()).unwrap())
            .await
            .unwrap();


        let (_parts, raw_body) = resp.into_parts();
        let body = hyper::body::to_bytes(raw_body).await.unwrap();
        let html = String::from_utf8(body.to_vec()).unwrap();
        let selector = Selector::parse("h1")
            .unwrap();
        let doc = Html::parse_document(&html);
        let el = doc.select(&selector).next().unwrap();
        assert_eq!("Welcome!", el.text().next().unwrap())
    }

    #[tokio::test]
    async fn test_static() {
        let app = get_app();

        let resp = app
            .oneshot(Request::builder().uri("/foo/bar").body(Body::empty()).unwrap())
            .await
            .unwrap();


        println!("{:?}", resp);
        assert_eq!(StatusCode::OK, resp.status());

    }
    // You can also spawn a server and talk to it like any other HTTP server:
    #[tokio::test]
    async fn the_real_deal() {
        let listener = TcpListener::bind("0.0.0.0:8080".parse::<SocketAddr>().unwrap()).unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            axum::Server::from_tcp(listener)
                .unwrap()
                .serve(get_app().into_make_service())
                .await
                .unwrap();
        });

        let client = hyper::Client::new();

        let url = format!("http://{}/foo/barz", addr);
        println!("{}", url);
        let response = client
            .request(
                Request::builder()
                    .uri(url)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(StatusCode::OK, response.status());
    }

}
