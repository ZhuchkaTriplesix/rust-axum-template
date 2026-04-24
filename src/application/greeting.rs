use crate::config::App;
use crate::domain::Greeting;

/// Trivial "welcome" use case: maps config + domain `Greeting` (example vertical slice).
#[derive(Clone)]
pub struct GreetingService {
    app: App,
}

impl GreetingService {
    pub fn new(app: App) -> Self {
        Self { app }
    }

    pub fn welcome(&self) -> Greeting {
        Greeting::for_app(&self.app.name, &self.app.env)
    }
}
