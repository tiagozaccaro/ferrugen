<!-- Section from Microsoft Pragmatic Rust Guidelines -->
# Pub Crate Mod Foo Pub Struct Foo

# pub(crate) mod foo { pub struct Foo; }
#[doc(inline)]
pub use foo::*;

// or

#[doc(inline)]
pub use foo::Foo;
```

![TEXT](M-DOC-INLINE_GOOD.png)

This does not apply to `std` or 3rd party types; these should always be re-exported without inlining to make it clear they are external.

> ### <alert></alert> Still avoid glob exports
>
> The `#[doc(inline)]` trick above does not change [M-NO-GLOB-REEXPORTS]; you generally should not re-export items via wildcards.

[M-NO-GLOB-REEXPORTS]: ../libs/resilience/#M-NO-GLOB-REEXPORTS



## First sentence is one line; approx. 15 words (M-FIRST-DOC-SENTENCE) { #M-FIRST-DOC-SENTENCE }

<why>easily skimmable API docs.</why>

When you document your item, the first sentence becomes the "summary sentence" that is extracted and shown in the module summary:

```rust
/// This is the summary sentence, shown in the module summary.
///
/// This is other documentation. It is only shown in that item's detail view.
/// Sentences here can be as long as you like and it won't cause any issues.
fn some_item() { }
```

Since Rust API documentation is rendered with a fixed max width, there is a naturally preferred sentence length you should not
exceed to keep things tidy on most screens.

If you keep things in a line, your docs will become easily skimmable. Compare, for example, the standard library:

![TEXT](M-FIRST-DOC-SENTENCE_GOOD.png)

Otherwise, you might end up with _widows_ and a generally unpleasant reading flow:

![TEXT](M-FIRST-DOC-SENTENCE_BAD.png)

As a rule of thumb, the first sentence should not exceed **15 words**.



## Has comprehensive module documentation (M-MODULE-DOCS) { #M-MODULE-DOCS }

<why>easy API docs navigation.</why>

Any public library module must have `//!` module documentation, and the first sentence must follow [M-DOC-FIRST-SENTENCE].

```rust,edition2021,ignore
pub mod ffi {
    //! Contains FFI abstractions.

    pub struct String {};
}
```

The rest of the module documentation should be comprehensive, i.e., cover the most relevant technical aspects of the contained items, including

- what the module contains
- when it should be used, possibly when not
- examples
- subsystem specifications (e.g., `std::fmt` [also describes its formatting language](https://doc.rust-lang.org/stable/std/fmt/index.html#formatting-parameters))
- observable side effects, including what guarantees are made about these, if any
- relevant implementation details, e.g., the used system APIs

 Great examples include:

- [`std::fmt`](https://doc.rust-lang.org/stable/std/fmt/index.html)
- [`std::pin`](https://doc.rust-lang.org/stable/std/pin/index.html)
- [`std::option`](https://doc.rust-lang.org/stable/std/option/index.html)

This does not mean every module should contain all of these items. But if there is something to say about the interaction of the contained types,
their module documentation is the right place.

[M-DOC-FIRST-SENTENCE]: ./#M-DOC-FIRST-SENTENCE


---


