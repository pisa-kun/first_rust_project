use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
pub struct HelloEvent {
    name: String,
}

#[derive(Debug, Serialize)]
pub struct HelloOutput {
    message: String,
}

pub fn hello(event: HelloEvent) -> HelloOutput {
    HelloOutput {
        message: format!("Hello, {}!", event.name),
    }
}