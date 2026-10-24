<!-- Section from Microsoft Pragmatic Rust Guidelines -->
# Foo Cargo Toml

# foo/Cargo.toml
[dependencies]
foo_macros = "=1.2.3"
```



## Macros don't lie about signatures (M-MACROS-DONT-LIE) { #M-MACROS-DONT-LIE }

<why>clarity for users and LLMs.</why>

Macros must not (make users) misrepresent signatures or the shape of items.

Macros have the ability to arbitrarily rewrite token streams. They could convert structs to enums, traits to functions, or perform any other transformation imaginable. They should, however, do none of that, as the resulting code will be highly confusing and virtually impossible to predict or reason about.

Among others, macros must not

- visibly convert the nature of data types (e.g., structs to enums, ...),
- alter function signatures,
- convert the `async`-ness of items,
- do anything else that materially detaches _what's written_ from _what's happening_.

```rust,ignore
// Bad: Adds extra parameter and marks function `async`. Impossible to 
// predict from reading code. 
#[magic_transform]
fn foo() { }

foo(token).await
```



## Proc macros should have separate impl crate incl. tests (M-PROC-IMPL) { #M-PROC-IMPL }

<why>thoroughly testable proc macros.</why>

Proc macros should be thin shims inside some `foo_proc` crate that delegate to a separate, regular library crate, usually called `foo_proc_impl`, which contains the actual token-stream transformation logic and its tests.

As proc macro crates are special, testing them from `foo_proc` usually requires workarounds for unit and snapshot tests. Instead, consider having a `foo_proc_impl` crate:

```rust,ignore
use proc_macro2::TokenStream;

pub fn my_macro(attr: TokenStream, item: TokenStream) -> TokenStream { ... }
```

These can come with regular [insta](https://insta.rs/) or similar snapshot tests, and are then exported as genuine proc macros via a `foo_proc` crate like so:

```rust,ignore
#[proc_macro_attribute]
pub fn my_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    foo_proc_impl::my_macro(attr.into(), item.into()).into()
}
```

The macros are then re-exported from the core crate:

```rust,ignore
pub use foo_proc::my_macro;
```

Inside the core crate, we also recommend adding [trybuild](https://docs.rs/trybuild/latest/trybuild/) UI tests with negative examples to ensure consistent error messages.



## Proc macros don't produce implied or hidden items (M-PROC-IMPLIED-ITEMS) { #M-PROC-IMPLIED-ITEMS }

<why>clear errors and correct hygiene and visibility.</why>

Macros should not define magic types on their own, in particular not public ones, or ones that don't rely on namespace tricks.

Some macros want to define types, for example

```rust,ignore
#[my_macro]
struct UserType;

// would expand to

struct UserType;
struct ExtraType; 
impl UserType {
    fn foo() -> ExtraType { ... };
}
```

This is almost always a bad idea for several reasons:

- they can conflict with existing user-defined types inside the same module,
- if done naively, they can conflict with other expansions of the same macro,
- they can clash with the user's naming conventions,
- they are invisible at source code level and easily forgotten to be re-exported where needed.

While it is possible for users to work around these limitations somewhat, these are paper cuts your users will have to deal with, possibly months after the fact when refactoring otherwise unrelated code.

Note that there is one exception to this rule that has generally acceptable UX, the overloaded use of [namespaces](https://doc.rust-lang.org/reference/names/namespaces.html) made prominent by crates like Rocket:

```rust,ignore
#[my_macro]
fn foo() { ... }

// would expand to

fn foo() { ... }

struct foo;
impl SomeTrait for foo { ... }
```

Here a new type `foo` is introduced with the same name as the function `foo`. Due to Rust's namespace rules they can co-exist and are automatically re-exported with their parent, and due to [Rust's casing rules (C-CASE)](https://rust-lang.github.io/api-guidelines/naming.html#casing-conforms-to-rfc-430-c-case) these are highly unlikely to clash with user-defined types. However, they would still not make for a pretty _public_ type, and are therefore mainly used inside root crates to define request handlers or FFI functions.

> ### <tip></tip> Namespaces != Modules
>
> Namespaces in Rust have nothing to do with namespaces in other languages. A namespace in C# is approximately a module in Rust. A namespace in Rust
is an esoteric property of names (e.g., `fn foo`, `struct Bar {}`, `moo!`) that decides which 'naming bucket' it lives in inside a module.


---


