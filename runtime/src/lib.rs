// Purpose: placeholder runtime; returns a string for now.
#![forbid(unsafe_code)]

pub fn ping() -> &'static str {
    "runtime::ok"
}