#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use auth::OpaqueTokenCodec;
use auth::TokenCodec;

#[test]
fn token_debug_does_not_leak() {
    let codec = OpaqueTokenCodec;
    let (token, _) = codec.generate();
    // token should not be empty
    assert!(!token.is_empty());
    // Debug of codec should not contain token
    let dbg = format!("{:?}", codec);
    assert!(!dbg.contains(&token));
}
