use axum::{
    body::Body,
    extract::Request,
    http::{header, HeaderValue, Method, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CorsRule {
    pub origin_pattern: String,
    pub action: CorsAction,
    pub methods: Vec<String>,
    pub headers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CorsAction {
    Allow,
    Deny,
}

#[derive(Debug, Clone)]
pub struct CorsConfig {
    rules: Vec<CorsRule>,
}

impl CorsConfig {
    /// Load CORS configuration from a file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read CORS config: {}", e))?;

        Self::from_str(&content)
    }

    /// Parse CORS configuration from a string
    pub fn from_str(content: &str) -> Result<Self, String> {
        let mut rules = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();

            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            // Parse the rule
            let origin_pattern = parts[0].to_string();

            // Default values
            let mut action = CorsAction::Allow;
            let mut methods = vec!["ALL".to_string()];
            let mut headers = vec!["ALL".to_string()];

            // Parse optional action (index 1)
            if parts.len() > 1 {
                match parts[1].to_uppercase().as_str() {
                    "ALLOW" => action = CorsAction::Allow,
                    "DENY" => action = CorsAction::Deny,
                    _ => {
                        eprintln!(
                            "Warning: Unknown action '{}' on line {}, defaulting to ALLOW",
                            parts[1],
                            line_num + 1
                        );
                    }
                }
            }

            // Parse optional methods (index 2)
            if parts.len() > 2 {
                let methods_str = parts[2];
                if methods_str.to_uppercase() != "ALL" {
                    methods = methods_str.split(',').map(|s| s.trim().to_uppercase()).collect();
                }
            }

            // Parse optional headers (index 3)
            if parts.len() > 3 {
                let headers_str = parts[3];
                if headers_str.to_uppercase() != "ALL" {
                    headers = headers_str.split(',').map(|s| s.trim().to_lowercase()).collect();
                }
            }

            rules.push(CorsRule {
                origin_pattern,
                action,
                methods,
                headers,
            });
        }

        Ok(CorsConfig { rules })
    }

    /// Check if an origin matches a pattern
    fn origin_matches_pattern(origin: &str, pattern: &str) -> bool {
        // Simple wildcard matching
        if pattern == "*" {
            return true;
        }

        // Support *.localhost or *.local patterns
        if pattern.starts_with("*.") {
            let suffix = &pattern[2..];
            return origin.ends_with(suffix);
        }

        // Exact match
        origin == pattern
    }

    /// Find the first matching rule for an origin
    pub fn find_rule(&self, origin: &str) -> Option<&CorsRule> {
        self.rules
            .iter()
            .find(|rule| Self::origin_matches_pattern(origin, &rule.origin_pattern))
    }

    /// Check if a request is allowed
    pub fn is_allowed(&self, origin: &str, method: Option<&str>) -> bool {
        let Some(rule) = self.find_rule(origin) else {
            // No rule found - deny by default
            return false;
        };

        // Check action
        if rule.action == CorsAction::Deny {
            return false;
        }

        // Check method if provided
        if let Some(method) = method {
            if !rule.methods.contains(&"ALL".to_string())
                && !rule.methods.contains(&method.to_uppercase()) {
                return false;
            }
        }

        true
    }

    /// Get allowed methods for an origin
    pub fn allowed_methods(&self, origin: &str) -> Vec<String> {
        let Some(rule) = self.find_rule(origin) else {
            return vec![];
        };

        if rule.action == CorsAction::Deny {
            return vec![];
        }

        if rule.methods.contains(&"ALL".to_string()) {
            vec!["GET".into(), "POST".into(), "PUT".into(), "DELETE".into(), "PATCH".into(), "OPTIONS".into()]
        } else {
            rule.methods.clone()
        }
    }

    /// Get allowed headers for an origin
    pub fn allowed_headers(&self, origin: &str) -> Vec<String> {
        let Some(rule) = self.find_rule(origin) else {
            return vec![];
        };

        if rule.action == CorsAction::Deny {
            return vec![];
        }

        rule.headers.clone()
    }

    /// Get all configured origins (for debugging)
    pub fn configured_origins(&self) -> Vec<String> {
        self.rules
            .iter()
            .map(|r| r.origin_pattern.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_allow() {
        let config = CorsConfig::from_str("http://localhost:3000").unwrap();
        assert_eq!(config.rules.len(), 1);
        assert_eq!(config.rules[0].action, CorsAction::Allow);
        assert!(config.is_allowed("http://localhost:3000", None));
    }

    #[test]
    fn test_parse_deny() {
        let config = CorsConfig::from_str("http://bad.com DENY").unwrap();
        assert_eq!(config.rules[0].action, CorsAction::Deny);
        assert!(!config.is_allowed("http://bad.com", None));
    }

    #[test]
    fn test_parse_methods() {
        let config = CorsConfig::from_str("http://localhost:3000 ALLOW GET,POST").unwrap();
        assert!(config.is_allowed("http://localhost:3000", Some("GET")));
        assert!(config.is_allowed("http://localhost:3000", Some("POST")));
        assert!(!config.is_allowed("http://localhost:3000", Some("DELETE")));
    }

    #[test]
    fn test_wildcard_pattern() {
        let config = CorsConfig::from_str("*.localhost ALLOW ALL").unwrap();
        assert!(config.is_allowed("app.localhost", None));
        assert!(config.is_allowed("api.localhost", None));
        assert!(!config.is_allowed("localhost", None));
    }

    #[test]
    fn test_first_match_wins() {
        let config = CorsConfig::from_str(
            "http://localhost:3000 DENY\nhttp://localhost:3000 ALLOW"
        ).unwrap();
        assert!(!config.is_allowed("http://localhost:3000", None));
    }

    #[test]
    fn test_comments_and_empty_lines() {
        let config = CorsConfig::from_str(
            "# Comment\n\nhttp://localhost:3000\n\n# Another comment"
        ).unwrap();
        assert_eq!(config.rules.len(), 1);
    }
}

/// CORS middleware function for Axum
pub async fn cors_middleware(
    config: Arc<CorsConfig>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Get the origin from the request (clone to avoid borrow issues)
    let origin = request
        .headers()
        .get(header::ORIGIN)
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_default();

    let method = request.method().clone();

    // Get requested headers for preflight (clone before moving request)
    let requested_headers = request
        .headers()
        .get(header::ACCESS_CONTROL_REQUEST_HEADERS)
        .cloned();

    // Handle preflight OPTIONS requests
    if method == Method::OPTIONS {
        // Check if origin is allowed
        if !config.is_allowed(&origin, None) {
            return Ok(StatusCode::FORBIDDEN.into_response());
        }

        let mut response = Response::builder()
            .status(StatusCode::NO_CONTENT)
            .body(Body::empty())
            .unwrap();

        // Add CORS headers
        let headers = response.headers_mut();

        if let Ok(origin_value) = HeaderValue::from_str(&origin) {
            headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin_value);
        }

        // Add allowed methods
        let methods = config.allowed_methods(&origin);
        if !methods.is_empty() {
            if let Ok(methods_value) = HeaderValue::from_str(&methods.join(", ")) {
                headers.insert(header::ACCESS_CONTROL_ALLOW_METHODS, methods_value);
            }
        }

        // Add allowed headers
        let allowed_headers = config.allowed_headers(&origin);
        if !allowed_headers.is_empty() {
            if allowed_headers.contains(&"ALL".to_string()) {
                // Allow all requested headers
                if let Some(req_headers) = requested_headers {
                    headers.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, req_headers);
                } else {
                    // Default common headers
                    if let Ok(default_headers) = HeaderValue::from_str("content-type, authorization, x-requested-with") {
                        headers.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, default_headers);
                    }
                }
            } else {
                if let Ok(headers_value) = HeaderValue::from_str(&allowed_headers.join(", ")) {
                    headers.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, headers_value);
                }
            }
        }

        // Max age for preflight cache
        if let Ok(max_age) = HeaderValue::from_str("86400") {
            headers.insert(header::ACCESS_CONTROL_MAX_AGE, max_age);
        }

        // Allow credentials
        if let Ok(creds) = HeaderValue::from_str("true") {
            headers.insert(header::ACCESS_CONTROL_ALLOW_CREDENTIALS, creds);
        }

        return Ok(response);
    }

    // For non-OPTIONS requests, check if origin is allowed
    if !origin.is_empty() {
        let method_str = method.as_str();
        if !config.is_allowed(&origin, Some(method_str)) {
            return Ok(StatusCode::FORBIDDEN.into_response());
        }
    }

    // Process the request
    let mut response = next.run(request).await;

    // Add CORS headers to response
    if !origin.is_empty() && config.is_allowed(&origin, None) {
        let headers = response.headers_mut();

        if let Ok(origin_value) = HeaderValue::from_str(&origin) {
            headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin_value);
        }

        if let Ok(creds) = HeaderValue::from_str("true") {
            headers.insert(header::ACCESS_CONTROL_ALLOW_CREDENTIALS, creds);
        }

        // Expose headers
        if let Ok(expose) = HeaderValue::from_str("*") {
            headers.insert(header::ACCESS_CONTROL_EXPOSE_HEADERS, expose);
        }
    }

    Ok(response)
}
