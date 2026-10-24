<!-- Section from Microsoft Pragmatic Rust Guidelines -->
# Macros

# Macros Guidelines



## Prefer 'macros by example' over proc macros (M-EXAMPLE-OVER-PROC) { #M-EXAMPLE-OVER-PROC }

<why>easy macro inspection and fast compilation.</why>

When a 'macro by example' can do the job, it should be preferred over proc macros.

Proc macros are more powerful, but their expansion can't easily be inspected. Where this versatility isn't needed, a simple 'macro by example' is the better option.

```rust,ignore
// Bad, attribute macro requires proc macro machinery, can be hard to 
// inspect in some IDEs, and isn't needed here.
#[make_new_id]
struct MyId;

// Good, easier to write, maintain and inspect, faster compilation speed.
make_new_id!(MyId);
```



## Third party items come from hidden `_private` module (M-MACRO-HELPERS) { #M-MACRO-HELPERS }

<why>predictable compilation.</why>

When a macro expansion needs to refer to third-party items, the host crate should re-export those from a hidden module, and the macro should emit fully-qualified paths through that module rather than expecting the user's crate to depend on the third-party crate directly.

For example, a crate `foo` requiring `bar` traits would do:

```rust,ignore
#[doc(hidden)]
pub mod _private {
    pub use ::bar::Bar;
}

pub use foo_proc::my_macro;
```

The `my_macro!` implementation would then rely on its presence in its emitted code:

```rust,ignore
impl ::foo::_private::Bar for MyType { ... }
```



## Macros are a last resort (M-MACRO-LAST-RESORT) { #M-MACRO-LAST-RESORT }

<why>minimal complexity.</why>

Macros should only be used if no other viable solution exists, compare this adage:

> As @littlecalculist always told me, “macros are for when you run out of language”. If you still have language left — and Rust gives you a lot of language — use the language first.
>
> @pcwalton

Macros are powerful, but come with several downsides. They

- are magic, and it can be impossible to predict what they do, or how they do it,
- disproportionally increase compilation time in projects that otherwise don't rely on them,
- can cause subtle breakage at edition boundaries where Rust syntax and semantics can change.

Counterintuitively, the more structurally complex the result of a macro expansion is, the worse an idea it is to use macros for that in the first place. The ideal macro makes your users go "_I know exactly what this will generate, but I don't want to write all of that by hand_".



## Macros assume main crate (M-MACRO-MAIN-CRATE) { #M-MACRO-MAIN-CRATE }

<why>simple macro logic.</why>

Procedural macros can (and should) assume they are used through their main crate and emit paths for that.

For crates including proc macros it is common to ship them split in 3 for technical reasons:

- `foo` - the main crate that re-exports macros from `foo_proc`, along with extra traits or types,
- `foo_proc` - facade re-exporting macros from `foo_proc_impl` with `proc-macro = true`,
- `foo_proc_impl` - the actual macro implementation and unit tests.

In some cases there can be additional crates involved. Authors might be tempted to make `foo`, `foo_proc`, and siblings all work, resulting in complex re-export hierarchies or the use of 3rd party helpers. In reality, the minimal UX gain is usually not worth the added complexity (or compile time overhead), given the ecosystem precedent of mostly not supporting these usage modes in the first place.

This also implies you should not attempt to support use cases where your crate is imported under a different name.



## Pin supporting proc macro crates (M-MACRO-VERSION-PIN) { #M-MACRO-VERSION-PIN }

<why>keep generated code compatible with its library</why>

A crate that re-exports macros from companion proc macro crates must pin them to its own exact
version via `=x.y.z`, and each macro crate must pin its own companion crates the same way. When
publishing, those crates are counted as one crate and must be published together with the same
exact version.

Without exact pins, a newer macro may generate code incompatible with an older version of the
crate, causing unexpected and hard-to-diagnose compilation failures in the generated code.

M-MACRO-VERSION-PIN does not apply to independently consumed macro libraries.

Example:

```toml
