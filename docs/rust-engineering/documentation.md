<!-- Section from Microsoft Pragmatic Rust Guidelines -->
# Documentation

# Documentation



## Documentation has canonical sections (M-CANONICAL-DOCS) { #M-CANONICAL-DOCS }

<why>established Rust documentation practices.</why>

Public library items must contain the canonical doc sections. The summary sentence must always be present. Extended documentation and examples
are strongly encouraged. The other sections must be present when applicable.

```rust
/// Summary sentence < 15 words.
///
/// Extended documentation in free form.
///
/// # Examples
/// One or more examples that show API usage like so.
///
/// # Errors
/// If fn returns `Result`, list known error conditions
///
/// # Panics
/// If fn may panic, list when this may happen
///
/// # Safety
/// If fn is `unsafe` or may otherwise cause UB, this section must list
/// all conditions a caller must uphold.
///
/// # Abort
/// If fn may abort the process, list when this may happen.
pub fn foo() {}
```

In contrast to other languages, you should not create a table of parameters. Instead parameter use is explained in plain text. In other words, do not

```rust,ignore
/// Copies a file.
///
/// # Parameters
/// - src: The source.
/// - dst: The destination.
fn copy(src: File, dst: File) {}
```

but instead:

```rust,ignore
/// Copies a file from `src` to `dst`.
fn copy(src: File, dst: File) {}
```

### Related Reading

- Function docs include error, panic, and safety considerations ([C-FAILURE](https://rust-lang.github.io/api-guidelines/documentation.html#c-failure))



## Mark `pub use` items with `#[doc(inline)]` (M-DOC-INLINE) { #M-DOC-INLINE }

<why>re-exported items that fit in with their siblings.</why>

When publicly re-exporting crate items via `pub use foo::Foo` or `pub use foo::*`, they show up in an opaque re-export block. In most cases, this is not
helpful to the reader:

![TEXT](M-DOC-INLINE_BAD.png)

Instead, you should annotate them with `#[doc(inline)]` at the `use` site, for them to be inlined organically:

```rust,edition2021,ignore
