<!-- Section from Microsoft Pragmatic Rust Guidelines -->
# Libraries Ux

# Libraries / UX Guidelines



## Functions are `async` over returning a Future (M-ASYNC-FN) { #M-ASYNC-FN }

<why>simpler code and easier-to-understand APIs.</why>

Functions should be declared `async fn foo()` over `fn foo() -> impl Future` when both are viable.

Functions marked `async` are more idiomatic and easier to read. An explicit `Future`-returning signature should only be used when required, for example inside traits or for _hot 'n heavy_ async functions, compare [M-ASYNC-STACK-SIZE](../../performance/#M-ASYNC-STACK-SIZE).

```rust,ignore
impl Foo {
    // Bad, signature is noisier and the body needs an extra `async` block
    fn foo() -> impl Future<Output = Result<T, E>> { async { Ok(t) } }

    // Good, method and implementation reads normally
    async fn foo() -> Result<T, E> { Ok(t) }
}
```



## Avoid smart pointers and wrappers in APIs (M-AVOID-WRAPPERS) { #M-AVOID-WRAPPERS }

<why>low cognitive load and ergonomic APIs.</why>

As a specialization of [M-ABSTRACTIONS-DONT-NEST], generic wrappers and smart pointers like
`Rc<T>`, `Arc<T>`, `Box<T>`, or `RefCell<T>` should be avoided in public APIs.

From a user perspective these are mostly implementation details, and introduce infectious complexity that users have to
resolve. In fact, these might even be impossible to resolve once multiple crates disagree about the required type of wrapper.

If wrappers are needed internally, they should be hidden behind a clean API that uses simple types like `&T`, `&mut T`, or `T` directly. Compare:

```rust,ignore
// Good: simple API
pub fn process_data(data: &Data) -> State { ... }
pub fn store_config(config: Config) -> Result<(), Error> { ... }

// Bad: Exposing implementation details
pub fn process_shared(data: Arc<Mutex<Shared>>) -> Box<Processed> { ... }
pub fn initialize(config: Rc<RefCell<Config>>) -> Arc<Server> { ... }
```

Smart pointers in APIs are acceptable when:

- The smart pointer is fundamental to the API's purpose (e.g., a new container lib)

- The smart pointer, based on benchmarks, significantly improves performance and the complexity is justified.

[M-ABSTRACTIONS-DONT-NEST]: ./#M-ABSTRACTIONS-DONT-NEST



## Modules are balanced in size and scope (M-BALANCED-MODULES) { #M-BALANCED-MODULES }

<why>discoverable functionality and clear API usage.</why>

Your module design should approximately follow established UX practices of menu design: A _reasonable_ number of your most important items should be placed in the crate root, and a comprehensible grouping of the remaining functionality into subordinate modules.

Two violations of that rule are encountered most frequently: flat module roots containing dozens of items without clear ordering, or the excessive use of submodules without items in the crate root. While there are crates where this makes sense (e.g., automatically generated `-sys` crates defining 100s of C items, or umbrella crates like `std` and `tokio`), the majority of library crates are not among them.

When designing your module layout, consider these factors:

- Essential items users must find in order to use a crate should go into its root. For example, a `foo_client` crate should probably have its main `Client` struct inside the root.
- Other items should be grouped semantically by use case. Modules named `traits` and `errors` don't help anyone, but `account`, `network` and `status` do.
- Also take into account that modules are the perfect place for module-level documentation that further explains the respective subsystem.



## Collections implement the appropriate iter traits (M-COLLECTION-TRAITS) { #M-COLLECTION-TRAITS }

<why>composable collections.</why>

Custom collections should implement the iterator-facing traits the standard library offers.

Whenever you define a new collection type `Collection<T>` for consumption by third parties, the following traits and types should also be implemented, [see here](https://cheats.rs/#iterators) for more details:

- the structs `IntoIter<T>`, `Iter<T>` and `IterMut<T>`,
- an `impl Iterator` for all of them,
- the methods `c.iter()` and `c.iter_mut()`,
- an `impl IntoIterator` for `Collection<T>`, `&Collection<T>` and `&mut Collection<T>`,
- an `impl FromIterator` for `Collection<T>`,
- an `Extend` for `Collection<T>`,
- `DoubleEndedIterator`, `ExactSizeIterator`, ... as applicable

In addition, make sure you implement `size_hint()` on all iterators and do so truthfully.



## Prefer types over generics, generics over dyn traits (M-DI-HIERARCHY) { #M-DI-HIERARCHY }

<why>composable patterns and freedom from design lock-in.</why>

When asking for async dependencies, prefer concrete types over generics, and generics over `dyn Trait`.

It is easy to accidentally deviate from this pattern when porting code from languages like C# that heavily rely on interfaces.
Consider you are porting a service called `Database` from C# to Rust and, inspired by the original `IDatabase` interface, you naively translate it into:

```rust,ignore
trait Database {
    async fn update_config(&self, file: PathBuf);
    async fn store_object(&self, id: Id, obj: Object);
    async fn load_object(&self, id: Id) -> Object;
}

impl Database for MyDatabase { ... }

// Intended to be used like this:
async fn start_service(b: Rc<dyn Database>) { ... }
```

Apart from not feeling idiomatic, this approach precludes other Rust constructs that conflict with object safety,
can cause issues with asynchronous code, and exposes wrappers (compare [M-AVOID-WRAPPERS]).

Instead, when more than one implementation is needed, this _design escalation ladder_ should be followed:

If the other implementation is only concerned with providing a _sans-io_ implementation for testing, implement your type as an
enum, following [M-MOCKABLE-SYSCALLS] instead.

If users are expected to provide custom implementations, you should introduce one or more traits, and implement them for your own types
_on top_ of your inherent functions. Each trait should be relatively narrow, e.g., `StoreObject`, `LoadObject`. If eventually a single
trait is needed it should be made a subtrait, e.g., `trait DataAccess: StoreObject + LoadObject {}`.

Code working with these traits should ideally accept them as generic type parameters as long as their use does not contribute to significant nesting
(compare [M-ABSTRACTIONS-DONT-NEST]).

```rust,ignore
// Good, generic does not have infectious impact, uses only most specific trait
async fn read_database(x: impl LoadObject) { ... }

// Acceptable, unless further nesting makes this excessive.
struct MyService<T: DataAccess> {
    db: T,
}
```

Once generics become a nesting problem, `dyn Trait` can be considered. Even in this case, visible wrapping should be avoided, and custom wrappers should be preferred.

```rust
