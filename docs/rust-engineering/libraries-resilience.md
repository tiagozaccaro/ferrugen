<!-- Section from Microsoft Pragmatic Rust Guidelines -->
# Libraries Resilience

# Libraries / Resilience Guidelines



## Avoid statics (M-AVOID-STATICS) { #M-AVOID-STATICS }

<why>consistency and correctness across crate versions.</why>

Libraries should avoid `static` and thread-local items, if a consistent view of the item is relevant for correctness.
Essentially, any code that would be incorrect if the static _magically_ had another value must not use them. Statics
only used for performance optimizations are ok.

The fundamental issue with statics in Rust is the secret duplication of state.

Consider a crate `core` with the following function:

```rust
