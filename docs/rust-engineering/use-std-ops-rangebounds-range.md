<!-- Section from Microsoft Pragmatic Rust Guidelines -->
# Use Std Ops Rangebounds Range

# use std::ops::{RangeBounds, Range};
// Callers must call with `select_range(1..3)`
fn select_range(r: Range<usize>) {}

// Callers may call as
//     select_any(1..3)
//     select_any(1..)
//     select_any(..)
fn select_any(r: impl RangeBounds<usize>) {}
```



## Types are Send (M-TYPES-SEND) { #M-TYPES-SEND }

<why>use in Tokio and behind runtime abstractions.</why>

Public types should be `Send` for compatibility reasons:

- All futures produced (explicitly or implicitly) must be `Send`
- Most other types should be `Send`, but there might be exceptions

### Futures

When declaring a future explicitly you should ensure it is, and remains, `Send`.

```rust
