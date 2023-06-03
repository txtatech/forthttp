use crate::server::HTML_CONTENT;
use std::sync::Mutex;
use crate::server::HTML_CONTENT as OtherHTML_CONTENT;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref STATIC_HTML_CONTENT: Mutex<String> = Mutex::new(String::new());
}

pub fn execute_forth(code: &str) -> String {
    let mut content = HTML_CONTENT.lock().unwrap();

    match code {
        "install" => {
            content.push_str("Installing Service Worker...\n");
        }
        "activate" => {
            content.push_str("Activating Service Worker...\n");
        }
        "fetch" => {
            content.push_str("Fetching resources with Service Worker...\n");
        }
        "+" => {
            // Perform addition
            let result = 2 + 3;  // Replace with your actual addition logic
            content.push_str(&format!("Result: {}\n", result));
        }
        "-" => {
            // Perform subtraction
            let result = 5 - 2;  // Replace with your actual subtraction logic
            content.push_str(&format!("Result: {}\n", result));
        }
        "*" => {
            // Perform multiplication
            let result = 4 * 6;  // Replace with your actual multiplication logic
            content.push_str(&format!("Result: {}\n", result));
        }
        "/" => {
            // Perform division
            let result = 10 / 2;  // Replace with your actual division logic
            content.push_str(&format!("Result: {}\n", result));
        }
        "." => {
            // Perform output/display logic
            let result = 42;  // Replace with the value you want to display
            content.push_str(&format!("Output: {}\n", result));
        }
        _ => {
            content.push_str("Unknown operation: ");
            content.push_str(code);
            content.push('\n');
        }
    }

    content.clone()
}

// Add any other necessary code or definitions below this point
