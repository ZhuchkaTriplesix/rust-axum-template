use serde::Serialize;

/// Minimal example of a value object / domain concept used by a use case.
#[derive(Debug, Clone, Serialize)]
pub struct Greeting {
    pub message: String,
    pub app_name: String,
    pub environment: String,
}

impl Greeting {
    pub fn for_app(app_name: &str, environment: &str) -> Self {
        Self {
            message: "Template is up — add your aggregates under `src/domain`.".to_string(),
            app_name: app_name.to_string(),
            environment: environment.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Greeting;

    #[test]
    fn welcome_contains_app() {
        let g = Greeting::for_app("demo", "test");
        assert_eq!(g.app_name, "demo");
        assert_eq!(g.environment, "test");
    }
}
