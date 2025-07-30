use poem::{Endpoint, IntoResponse, Middleware, Request, Response, Result, http::StatusCode};
use serde_json;
use std::fmt;

use crate::models::responses::ErrorResponse;

/// The default API key that the mock server accepts
pub const DEFAULT_MOCK_API_KEY: &str = "sk-mock-openai-api-key-12345";

/// Authentication middleware that validates API keys
#[derive(Clone)]
pub struct AuthMiddleware {
    api_key: String,
}

impl AuthMiddleware {
    /// Create a new authentication middleware instance with the specified API key
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
        }
    }

    /// Create a new authentication middleware instance with the default API key
    pub fn with_default_key() -> Self {
        Self::new(DEFAULT_MOCK_API_KEY)
    }
}

impl Default for AuthMiddleware {
    fn default() -> Self {
        Self::with_default_key()
    }
}

impl<E> Middleware<E> for AuthMiddleware
where
    E: Endpoint,
{
    type Output = AuthEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        AuthEndpoint {
            inner: ep,
            api_key: self.api_key.clone(),
        }
    }
}

/// Wrapper endpoint that performs authentication
pub struct AuthEndpoint<E> {
    inner: E,
    api_key: String,
}

impl<E> Endpoint for AuthEndpoint<E>
where
    E: Endpoint,
{
    type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        // Check for Authorization header
        let auth_header = match req.headers().get("authorization") {
            Some(header) => header,
            None => {
                return Ok(create_auth_error_response(
                    "You didn't provide an API key. You need to provide your API key in an Authorization header using Bearer auth (i.e. Authorization: Bearer YOUR_KEY), or as the password field (with blank username) if you're accessing the API from your browser and are prompted for a username and password. You can obtain an API key from https://platform.openai.com/account/api-keys.",
                ));
            }
        };

        // Convert header to string
        let auth_str = match auth_header.to_str() {
            Ok(s) => s,
            Err(_) => {
                return Ok(create_auth_error_response(
                    "Invalid Authorization header format.",
                ));
            }
        };

        // Check Bearer token format
        if !auth_str.starts_with("Bearer ") {
            return Ok(create_auth_error_response(
                "You must provide the API key using Bearer authentication (i.e. Authorization: Bearer YOUR_KEY).",
            ));
        }

        // Extract the token part after "Bearer "
        let token = &auth_str[7..]; // "Bearer " is 7 characters

        // Validate the API key
        if token != self.api_key {
            return Ok(create_auth_error_response(
                "Incorrect API key provided: ***. You can find your API key at https://platform.openai.com/account/api-keys.",
            ));
        }

        // Authentication successful, proceed to the next endpoint
        match self.inner.call(req).await {
            Ok(response) => Ok(response.into_response()),
            Err(err) => Err(err),
        }
    }
}

/// Authentication error type
#[derive(Debug)]
pub enum AuthError {
    MissingHeader,
    InvalidFormat,
    InvalidKey,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::MissingHeader => write!(f, "Missing Authorization header"),
            AuthError::InvalidFormat => write!(f, "Invalid Authorization header format"),
            AuthError::InvalidKey => write!(f, "Invalid API key"),
        }
    }
}

impl std::error::Error for AuthError {}

/// Create a standardized authentication error response
fn create_auth_error_response(message: &str) -> Response {
    let error_response =
        ErrorResponse::new(message.to_string(), "invalid_request_error".to_string());

    let json_body = match serde_json::to_string(&error_response) {
        Ok(json) => json,
        Err(_) => r#"{"error":{"message":"Authentication failed","type":"invalid_request_error"}}"#
            .to_string(),
    };

    Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .header("content-type", "application/json")
        .body(json_body)
}

/// Helper function to extract and validate API key from request
pub fn extract_api_key(req: &Request, expected_key: &str) -> Result<String, AuthError> {
    let auth_header = req
        .headers()
        .get("authorization")
        .ok_or(AuthError::MissingHeader)?;

    let auth_str = auth_header.to_str().map_err(|_| AuthError::InvalidFormat)?;

    if !auth_str.starts_with("Bearer ") {
        return Err(AuthError::InvalidFormat);
    }

    let token = &auth_str[7..];

    if token != expected_key {
        return Err(AuthError::InvalidKey);
    }

    Ok(token.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use poem::{
        EndpointExt, Route, handler,
        http::{Method, Uri},
    };

    #[handler]
    async fn test_handler() -> &'static str {
        "success"
    }

    #[tokio::test]
    async fn test_missing_authorization_header() {
        let app = Route::new()
            .at("/test", poem::get(test_handler))
            .with(AuthMiddleware::with_default_key());

        let req = Request::builder()
            .method(Method::GET)
            .uri(Uri::from_static("/test"))
            .finish();

        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

        let body = resp.into_body().into_string().await.unwrap();
        assert!(body.contains("You didn't provide an API key"));
        assert!(body.contains("invalid_request_error"));
    }

    #[tokio::test]
    async fn test_invalid_authorization_format() {
        let app = Route::new()
            .at("/test", poem::get(test_handler))
            .with(AuthMiddleware::with_default_key());

        let req = Request::builder()
            .method(Method::GET)
            .uri(Uri::from_static("/test"))
            .header("authorization", "Basic dGVzdA==")
            .finish();

        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

        let body = resp.into_body().into_string().await.unwrap();
        assert!(body.contains("Bearer authentication"));
    }

    #[tokio::test]
    async fn test_invalid_api_key() {
        let app = Route::new()
            .at("/test", poem::get(test_handler))
            .with(AuthMiddleware::with_default_key());

        let req = Request::builder()
            .method(Method::GET)
            .uri(Uri::from_static("/test"))
            .header("authorization", "Bearer sk-invalid-key")
            .finish();

        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

        let body = resp.into_body().into_string().await.unwrap();
        assert!(body.contains("Incorrect API key provided"));
    }

    #[tokio::test]
    async fn test_valid_api_key() {
        let app = Route::new()
            .at("/test", poem::get(test_handler))
            .with(AuthMiddleware::with_default_key());

        let req = Request::builder()
            .method(Method::GET)
            .uri(Uri::from_static("/test"))
            .header("authorization", format!("Bearer {DEFAULT_MOCK_API_KEY}"))
            .finish();

        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let body = resp.into_body().into_string().await.unwrap();
        assert_eq!(body, "success");
    }

    #[tokio::test]
    async fn test_extract_api_key_success() {
        let req = Request::builder()
            .header("authorization", format!("Bearer {DEFAULT_MOCK_API_KEY}"))
            .finish();

        let result = extract_api_key(&req, DEFAULT_MOCK_API_KEY);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), DEFAULT_MOCK_API_KEY);
    }

    #[tokio::test]
    async fn test_extract_api_key_missing_header() {
        let req = Request::builder().finish();
        let result = extract_api_key(&req, DEFAULT_MOCK_API_KEY);
        assert!(matches!(result, Err(AuthError::MissingHeader)));
    }

    #[tokio::test]
    async fn test_extract_api_key_invalid_format() {
        let req = Request::builder()
            .header("authorization", "Basic dGVzdA==")
            .finish();

        let result = extract_api_key(&req, DEFAULT_MOCK_API_KEY);
        assert!(matches!(result, Err(AuthError::InvalidFormat)));
    }

    #[tokio::test]
    async fn test_extract_api_key_invalid_key() {
        let req = Request::builder()
            .header("authorization", "Bearer sk-invalid-key")
            .finish();

        let result = extract_api_key(&req, DEFAULT_MOCK_API_KEY);
        assert!(matches!(result, Err(AuthError::InvalidKey)));
    }

    #[test]
    fn test_auth_error_display() {
        assert_eq!(
            format!("{}", AuthError::MissingHeader),
            "Missing Authorization header"
        );
        assert_eq!(
            format!("{}", AuthError::InvalidFormat),
            "Invalid Authorization header format"
        );
        assert_eq!(format!("{}", AuthError::InvalidKey), "Invalid API key");
    }

    #[tokio::test]
    async fn test_custom_api_key() {
        let custom_key = "sk-custom-test-key-12345";
        let app = Route::new()
            .at("/test", poem::get(test_handler))
            .with(AuthMiddleware::new(custom_key));

        // Test with custom key - should succeed
        let req = Request::builder()
            .method(Method::GET)
            .uri(Uri::from_static("/test"))
            .header("authorization", format!("Bearer {custom_key}"))
            .finish();

        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // Test with default key - should fail
        let req = Request::builder()
            .method(Method::GET)
            .uri(Uri::from_static("/test"))
            .header("authorization", format!("Bearer {DEFAULT_MOCK_API_KEY}"))
            .finish();

        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_auth_middleware_creation() {
        let custom_key = "sk-test-key";
        let middleware = AuthMiddleware::new(custom_key);
        assert_eq!(middleware.api_key, custom_key);

        let default_middleware = AuthMiddleware::with_default_key();
        assert_eq!(default_middleware.api_key, DEFAULT_MOCK_API_KEY);

        let default_via_trait = AuthMiddleware::default();
        assert_eq!(default_via_trait.api_key, DEFAULT_MOCK_API_KEY);
    }

    #[tokio::test]
    async fn test_extract_api_key_with_custom_key() {
        let custom_key = "sk-custom-key-123";

        // Test with matching custom key
        let req = Request::builder()
            .header("authorization", format!("Bearer {custom_key}"))
            .finish();

        let result = extract_api_key(&req, custom_key);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), custom_key);

        // Test with non-matching key
        let result = extract_api_key(&req, "different-key");
        assert!(matches!(result, Err(AuthError::InvalidKey)));
    }
}
