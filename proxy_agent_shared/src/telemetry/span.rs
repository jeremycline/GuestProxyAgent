use std::fmt::{Display, Formatter};
// Copyright (c) Microsoft Corporation
// SPDX-License-Identifier: MIT
use serde_derive::{Deserialize, Serialize};
use std::time::Instant;

pub struct SimpleSpan {
    start: Instant,
}

#[derive(Serialize, Deserialize)]
struct ElapsedMessage {
    elapsed: u128,
    message: String,
}

impl ElapsedMessage {
    fn new(elapsed: u128, message: String) -> Self {
        ElapsedMessage { elapsed, message }
    }

    fn to_json_string(&self) -> String {
        format!(
            "{{\"elapsed\":{}, \"message\":\"{}\"}}",
            self.elapsed, self.message
        )
    }
}

impl Display for ElapsedMessage {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{} - {}", self.message, self.elapsed)
    }
}

impl Default for SimpleSpan {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleSpan {
    pub fn new() -> Self {
        SimpleSpan {
            start: Instant::now(),
        }
    }

    pub fn start_new(&mut self) {
        self.start = Instant::now();
    }

    pub fn get_elapsed_time_in_millisec(&self) -> u128 {
        self.start.elapsed().as_millis()
    }

    pub fn get_elapsed_json_message(&self, message: &str) -> String {
        let elapsed_massage =
            ElapsedMessage::new(self.get_elapsed_time_in_millisec(), message.to_string());
        elapsed_massage.to_json_string()
    }

    pub fn write_event(
        &self,
        message: &str,
        _method_name: &str,
        _module_name: &str,
        _logger_key: &str,
    ) -> String {
        let elapsed_massage =
            ElapsedMessage::new(self.get_elapsed_time_in_millisec(), message.to_string())
                .to_string();
        tracing::info!(elapsed_massage,);
        elapsed_massage
    }
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn span_test() {
        let mut span = super::SimpleSpan::new();
        sleep(Duration::from_millis(1));
        let elapsed = span.get_elapsed_time_in_millisec();
        assert!(elapsed > 0);
        let duration = Duration::from_millis(100);
        sleep(duration);
        let message: String = span.get_elapsed_json_message("test");
        let elapsed_message: super::ElapsedMessage = serde_json::from_str(&message).unwrap();
        assert_eq!(elapsed_message.message, "test");
        assert!(elapsed_message.elapsed > duration.as_millis());

        span.start_new();
        sleep(Duration::from_millis(1));
        let elapsed = span.get_elapsed_time_in_millisec();
        assert!(elapsed > 0);
        assert!(elapsed < duration.as_millis());
    }
}
