use axum::response::{IntoResponse, Response, Html};
use axum::{Json, Router};
use axum::routing::get;
use askama::Template;
use axum::http::StatusCode;

#[derive(Template)]
#[template(path = "welcome.html")]
struct WelcomeTmpl {
    name: String
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
    let app = Router::new()
        .route("/ping", get(ping))
        .route("/welcome", get(welcome));

    app
}

async fn welcome() -> impl IntoResponse {
    let tmpl = WelcomeTmpl { name: "hey there".to_string() };
    HtmlTemplate(tmpl)
}

async fn ping() -> impl IntoResponse {
   Json("pong")
}

#[cfg(test)]
mod tests {
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

}
