//! Wire protocol, version 0.
//!
//! Every message is a JSON object with three fixed fields (`protocol`, `id`,
//! `type`) plus one variable `body`. The shape of `body` depends on `type`.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use ulid::Ulid;

/// Version of the wire protocol implemented by this module.
pub const PROTOCOL_VERSION: u16 = 0;

fn empty_object() -> Value {
    Value::Object(serde_json::Map::new())
}

/// A protocol message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Envelope {
    /// Protocol version, currently [`PROTOCOL_VERSION`].
    pub protocol: u16,
    /// Unique message id (ULID).
    pub id: Ulid,
    /// Type-specific message.
    #[serde(flatten)]
    pub message: Message,
}

/// The variable part of the envelope, tagged by `type`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Message {
    /// Unauthenticated or system-level call.
    Request { body: RequestBody },
    /// Authenticated game command.
    Command { body: CommandBody },
    /// Reply to a request or command.
    Response { body: ResponseBody },
    /// Server broadcast.
    Event { body: EventBody },
}

/// Body of a `request` message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestBody {
    /// Namespaced method name, for example `system.ping`.
    pub name: String,
    /// Method arguments.
    #[serde(default = "empty_object")]
    pub args: Value,
}

/// Body of a `command` message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandBody {
    /// Namespaced command name, for example `clubs.claim`.
    pub name: String,
    /// Session that issued the command.
    pub session_id: Ulid,
    /// User that issued the command.
    pub user_id: Ulid,
    /// Idempotency key chosen by the client.
    pub command_id: Ulid,
    /// World version the client expects.
    pub expected_version: u64,
    /// Command arguments.
    #[serde(default = "empty_object")]
    pub args: Value,
}

/// Body of a `response` message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseBody {
    /// Id of the request or command being answered.
    pub reply_to: Ulid,
    /// Whether the operation succeeded.
    pub ok: bool,
    /// Result payload when `ok` is true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    /// Error payload when `ok` is false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorBody>,
}

/// Error payload of a failed response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorBody {
    /// Stable machine-readable code, for example `conflict`.
    pub code: String,
    /// Human-readable message.
    pub message: String,
    /// Optional structured details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
}

/// Body of an `event` message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventBody {
    /// Namespaced event name, for example `club.claimed`.
    pub name: String,
    /// Monotonic sequence number assigned by the server.
    pub seq: u64,
    /// World that produced the event.
    pub world_id: Ulid,
    /// Event payload.
    #[serde(default = "empty_object")]
    pub data: Value,
}

impl Envelope {
    /// Builds a `request` envelope with the current protocol version.
    pub fn request(id: Ulid, body: RequestBody) -> Self {
        Self {
            protocol: PROTOCOL_VERSION,
            id,
            message: Message::Request { body },
        }
    }

    /// Builds a `command` envelope with the current protocol version.
    pub fn command(id: Ulid, body: CommandBody) -> Self {
        Self {
            protocol: PROTOCOL_VERSION,
            id,
            message: Message::Command { body },
        }
    }

    /// Builds a `response` envelope with the current protocol version.
    pub fn response(id: Ulid, body: ResponseBody) -> Self {
        Self {
            protocol: PROTOCOL_VERSION,
            id,
            message: Message::Response { body },
        }
    }

    /// Builds an `event` envelope with the current protocol version.
    pub fn event(id: Ulid, body: EventBody) -> Self {
        Self {
            protocol: PROTOCOL_VERSION,
            id,
            message: Message::Event { body },
        }
    }

    /// Checks the protocol version and type-specific invariants.
    pub fn validate(&self) -> Result<(), ProtocolError> {
        if self.protocol != PROTOCOL_VERSION {
            return Err(ProtocolError::UnsupportedVersion {
                found: self.protocol,
                expected: PROTOCOL_VERSION,
            });
        }

        if let Message::Response { body } = &self.message {
            body.validate()?;
        }

        Ok(())
    }
}

impl ResponseBody {
    /// Builds a successful response.
    pub fn success(reply_to: Ulid, result: Value) -> Self {
        Self {
            reply_to,
            ok: true,
            result: Some(result),
            error: None,
        }
    }

    /// Builds a failed response.
    pub fn failure(reply_to: Ulid, error: ErrorBody) -> Self {
        Self {
            reply_to,
            ok: false,
            result: None,
            error: Some(error),
        }
    }

    /// Ensures `ok`, `result` and `error` are consistent.
    pub fn validate(&self) -> Result<(), ProtocolError> {
        let has_result = self.result.is_some();
        let has_error = self.error.is_some();
        let valid = if self.ok {
            !has_error
        } else {
            has_error && !has_result
        };

        if valid {
            Ok(())
        } else {
            Err(ProtocolError::InvalidResponse {
                ok: self.ok,
                has_result,
                has_error,
            })
        }
    }
}

impl ErrorBody {
    /// Builds an error payload.
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }
}

/// Errors produced while validating a protocol message.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProtocolError {
    /// The envelope declares a protocol version this build does not support.
    #[error("unsupported protocol version {found}, expected {expected}")]
    UnsupportedVersion { found: u16, expected: u16 },

    /// A response has an inconsistent combination of `ok`, `result` and `error`.
    #[error("invalid response: ok={ok}, result={has_result}, error={has_error}")]
    InvalidResponse {
        ok: bool,
        has_result: bool,
        has_error: bool,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const ID: &str = "01JB7Y6Q3M8ZC4R5T9V2K1X0AB";
    const SESSION: &str = "01JB7Y6Q3M8ZC4R5T9V2K1X0AC";
    const USER: &str = "01JB7Y6Q3M8ZC4R5T9V2K1X0AD";
    const COMMAND: &str = "01JB7Y6Q3M8ZC4R5T9V2K1X0AE";
    const CLUB: &str = "01JB7Y6Q3M8ZC4R5T9V2K1X0AF";
    const WORLD: &str = "01JB7Y6Q3M8ZC4R5T9V2K1X0AK";

    fn ulid(value: &str) -> Ulid {
        Ulid::from_string(value).expect("test ULID must be valid")
    }

    #[test]
    fn request_round_trip() {
        let value = json!({
            "protocol": 0,
            "id": ID,
            "type": "request",
            "body": {
                "name": "system.ping",
                "args": {}
            }
        });

        let envelope: Envelope = serde_json::from_value(value.clone()).unwrap();

        assert!(envelope.validate().is_ok());
        assert_eq!(serde_json::to_value(&envelope).unwrap(), value);
    }

    #[test]
    fn command_round_trip() {
        let value = json!({
            "protocol": 0,
            "id": ID,
            "type": "command",
            "body": {
                "name": "clubs.claim",
                "session_id": SESSION,
                "user_id": USER,
                "command_id": COMMAND,
                "expected_version": 42,
                "args": { "club_id": CLUB }
            }
        });

        let envelope: Envelope = serde_json::from_value(value.clone()).unwrap();

        assert!(envelope.validate().is_ok());
        assert_eq!(serde_json::to_value(&envelope).unwrap(), value);
    }

    #[test]
    fn successful_response_round_trip() {
        let value = json!({
            "protocol": 0,
            "id": ID,
            "type": "response",
            "body": {
                "reply_to": COMMAND,
                "ok": true,
                "result": { "pong": true }
            }
        });

        let envelope: Envelope = serde_json::from_value(value.clone()).unwrap();

        assert!(envelope.validate().is_ok());
        assert_eq!(serde_json::to_value(&envelope).unwrap(), value);
    }

    #[test]
    fn failed_response_round_trip() {
        let value = json!({
            "protocol": 0,
            "id": ID,
            "type": "response",
            "body": {
                "reply_to": COMMAND,
                "ok": false,
                "error": {
                    "code": "conflict",
                    "message": "expected version 42, current 43"
                }
            }
        });

        let envelope: Envelope = serde_json::from_value(value.clone()).unwrap();

        assert!(envelope.validate().is_ok());
        assert_eq!(serde_json::to_value(&envelope).unwrap(), value);
    }

    #[test]
    fn event_round_trip() {
        let value = json!({
            "protocol": 0,
            "id": ID,
            "type": "event",
            "body": {
                "name": "club.claimed",
                "seq": 1235,
                "world_id": WORLD,
                "data": { "club_id": CLUB, "user_id": USER }
            }
        });

        let envelope: Envelope = serde_json::from_value(value.clone()).unwrap();

        assert!(envelope.validate().is_ok());
        assert_eq!(serde_json::to_value(&envelope).unwrap(), value);
    }

    #[test]
    fn rejects_unsupported_protocol_version() {
        let mut envelope = Envelope::request(
            ulid(ID),
            RequestBody {
                name: "system.ping".to_owned(),
                args: json!({}),
            },
        );
        envelope.protocol = 1;

        assert_eq!(
            envelope.validate(),
            Err(ProtocolError::UnsupportedVersion {
                found: 1,
                expected: PROTOCOL_VERSION,
            })
        );
    }

    #[test]
    fn rejects_failed_response_without_error() {
        let body = ResponseBody {
            reply_to: ulid(COMMAND),
            ok: false,
            result: None,
            error: None,
        };

        assert!(body.validate().is_err());
    }

    #[test]
    fn rejects_successful_response_with_error() {
        let body = ResponseBody {
            reply_to: ulid(COMMAND),
            ok: true,
            result: None,
            error: Some(ErrorBody::new("boom", "unexpected")),
        };

        assert!(body.validate().is_err());
    }
}
