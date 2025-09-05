// Purpose: minimal facade so downstream crates can compile immediately.
// We'll replace with a real tokenizer in Phase 5
#![forbid(unsafe_code)]

pub fs ping() -> &'static str {
    "lexer::ok"
}