abstractions-don-t-visibly-nest-m-simple-abstractions.md
accept `impl 'io'` where feasible ('sans io') (m-impl-io).md
accept `impl asref<>` where feasible (m-impl-asref).md
accept `impl rangebounds<>` where feasible (m-impl-rangebounds).md
accept-impl-asref-where-feasible-m-impl-asref.md
accept-impl-io-where-feasible-sans-io-m-impl-io.md
accept-impl-rangebounds-where-feasible-m-impl-rangebounds.md
add-other-platforms-here-based-on-needs.md
additional-lints-are-from-the-restriction-and-nursery-group-that-prevent-specific.md
ai.md
all code must be sound (m-unsound).md
all crates are siblings in one folder (m-crates-flat-folder).md
all-code-must-be-sound-m-unsound.md
all-crates-are-siblings-in-one-folder-m-crates-flat-folder.md
application.md
applications may use anyhow or derivatives (m-app-error).md
applications target highest viable target-cpu (m-target-cpu).md
applications-may-use-anyhow-or-derivatives-m-app-error.md
applications-target-highest-viable-target-cpu-m-target-cpu.md
async-fn-read-file-x-str.md
avoid meta design documentation (m-no-meta-design-documentation).md
avoid statics (m-avoid-statics).md
avoid-meta-design-documentation-m-no-meta-design-documentation.md
avoid-smart-pointers-and-wrappers-in-apis-m-avoid-wrappers.md
avoid-statics-m-avoid-statics.md
backtrace-backtrace.md
bad-crate-links-its-sibling-directly.md
builders validate in final `.build()` (m-build-result).md
builders-validate-in-final-build-m-build-result.md
business logic belongs in core crates, ffi only translates (m-ffi-translates).md
business-logic-belongs-in-core-crates-ffi-only-translates-m-ffi-translates.md
canonical-error-conversion-uses-from-not-map-err-m-from-error.md
collections are created with sufficient initial capacity (m-initial-capacity).md
collections-are-created-with-sufficient-initial-capacity-m-initial-capacity.md
collections-implement-the-appropriate-iter-traits-m-collection-traits.md
common settings come from the workspace cargo.toml (m-cargo-workspace).md
common-settings-come-from-the-workspace-cargo-toml-m-cargo-workspace.md
complex-type-construction-has-builders-m-init-builder.md
complex-type-initialization-hierarchies-are-cascaded-m-init-cascaded.md
constructs-being-used-in-source-code-in-order-to-drive-up-consistency.md
correctness.md
custom panics have a helpful message (m-panic-message).md
custom-panics-have-a-helpful-message-m-panic-message.md
define-custom-opt-outs-here.md
derive-debug.md
design with ai use in mind (m-design-for-ai).md
design-with-ai-use-in-mind-m-design-for-ai.md
detected programming bugs are panics, not errors (m-panic-on-bug).md
detected-programming-bugs-are-panics-not-errors-m-panic-on-bug.md
documentation has canonical sections (m-canonical-docs).md
documentation-has-canonical-sections-m-canonical-docs.md
documentation.md
don't leak external types (m-dont-leak-types).md
don-t-define-preludes-m-no-prelude.md
don-t-glob-re-export-items-m-no-glob-reexports.md
don-t-leak-external-types-m-dont-leak-types.md
errors-are-canonical-structs-m-errors-canonical-structs.md
essential-functionality-should-be-inherent-m-essential-fn-inherent.md
features are additive (m-features-additive).md
features-are-additive-m-features-additive.md
ffi crates follow established naming conventions (m-ffi-naming).md
ffi-crates-follow-established-naming-conventions-m-ffi-naming.md
ffi.md
first sentence is one line; approx. 15 words (m-first-doc-sentence).md
first-sentence-is-one-line-approx-15-words-m-first-doc-sentence.md
fn-download-file-self-url-impl-asref-str.md
fn-fmt-self-f-mut-formatter-std-fmt-result.md
fn-foo-self.md
fn-telemetry-telemetry-telemetry.md
follow the upstream guidelines (m-upstream-guidelines).md
follow-the-upstream-guidelines-m-upstream-guidelines.md
foo-cargo-toml.md
functions-are-async-over-returning-a-future-m-async-fn.md
good-going-through-workspace.md
has comprehensive module documentation (m-module-docs).md
has-comprehensive-module-documentation-m-module-docs.md
hot `async` functions reduce stack size (m-async-stack-size).md
hot-async-functions-reduce-stack-size-m-async-stack-size.md
i-o-and-system-calls-are-mockable-m-mockable-syscalls.md
ideal-for-most-workspaces.md
identify, profile, optimize the hot path early (m-hotpath).md
identify-profile-optimize-the-hot-path-early-m-hotpath.md
if in doubt, split the crate (m-smaller-crates).md
if-in-doubt-split-the-crate-m-smaller-crates.md
integration-tests-live-under-tests-m-integration-tests.md
isolate dll state between ffi libraries (m-isolate-dll-state).md
isolate-dll-state-between-ffi-libraries-m-isolate-dll-state.md
items are only visible through one path (m-single-item-path).md
items come from their original crate (m-foreign-reexports).md
items-are-only-visible-through-one-path-m-single-item-path.md
items-come-from-their-original-crate-m-foreign-reexports.md
libraries work out of the box (m-oobe).md
libraries-building.md
libraries-interoperability.md
libraries-resilience.md
libraries-ux.md
libraries-work-out-of-the-box-m-oobe.md
library telemetry does not tank performance (m-log-overhead).md
library-telemetry-does-not-tank-performance-m-log-overhead.md
library.md
lint overrides should use `#[expect]` (m-lint-override-expect).md
lint-overrides-should-use-expect-m-lint-override-expect.md
long-running tasks should have yield points (m-yield-points).md
long-running-tasks-should-have-yield-points-m-yield-points.md
macros are a last resort (m-macro-last-resort).md
macros assume main crate (m-macro-main-crate).md
macros don't lie about signatures (m-macros-dont-lie).md
macros-are-a-last-resort-m-macro-last-resort.md
macros-assume-main-crate-m-macro-main-crate.md
macros-don-t-lie-about-signatures-m-macros-dont-lie.md
macros.md
magic values are documented (m-documented-magic).md
magic-values-are-documented-m-documented-magic.md
mark `pub use` items with `#[doc(inline)]` (m-doc-inline).md
mark-pub-use-items-with-doc-inline-m-doc-inline.md
may-cause-issues-with-structured-logging-otherwise.md
modules-are-balanced-in-size-and-scope-m-balanced-modules.md
msrv is conservatively updated (m-msrv).md
msrv-is-conservatively-updated-m-msrv.md
names are free of weasel words (m-weasel-words).md
names of items are short (m-short-names).md
names-are-free-of-weasel-words-m-weasel-words.md
names-of-items-are-short-m-short-names.md
native `-sys` crates compile without dependencies (m-sys-crates).md
native escape hatches (m-escape-hatches).md
native-escape-hatches-m-escape-hatches.md
native-sys-crates-compile-without-dependencies-m-sys-crates.md
nested type hierarchies should avoid needless indirection (m-avoid-indirection).md
nested-type-hierarchies-should-avoid-needless-indirection-m-avoid-indirection.md
never-acceptable-crates-inside-src-folder.md
new crates target latest edition (m-latest-edition).md
new-crates-target-latest-edition-m-latest-edition.md
newtypes-guard-their-invariants-m-strong-types-guard.md
nursery-level-warn-priority-1-optional-might-cause-more-false-positives.md
ok-for-large-workspaces.md
optimize for throughput, avoid empty cycles (m-throughput).md
optimize-for-throughput-avoid-empty-cycles-m-throughput.md
panic continuation is last resort (m-panic-continuation).md
panic means 'stop the program' (m-panic-is-stop).md
panic-continuation-is-last-resort-m-panic-continuation.md
panic-means-stop-the-program-m-panic-is-stop.md
parameter-ordering-is-consistent-m-parameter-consistency.md
performance.md
pin supporting proc macro crates (m-macro-version-pin).md
pin-supporting-proc-macro-crates-m-macro-version-pin.md
pragmatic-rust-guidelines.md
preface.md
prefer 'macros by example' over proc macros (m-example-over-proc).md
prefer regular over associated functions (m-regular-fn).md
prefer-macros-by-example-over-proc-macros-m-example-over-proc.md
prefer-regular-over-associated-functions-m-regular-fn.md
prefer-types-over-generics-generics-over-dyn-traits-m-di-hierarchy.md
proc macros don't produce implied or hidden items (m-proc-implied-items).md
proc macros should have separate impl crate incl. tests (m-proc-impl).md
proc-macros-don-t-produce-implied-or-hidden-items-m-proc-implied-items.md
proc-macros-should-have-separate-impl-crate-incl-tests-m-proc-impl.md
production-code-uses-telemetry-not-println-m-log-not-print.md
project.md
pub-crate-mod-foo-pub-struct-foo.md
pub-struct-configurationerror.md
public types are debug (m-public-debug).md
public types meant to be read are display (m-public-display).md
public-types-are-debug-m-public-debug.md
public-types-meant-to-be-read-are-display-m-public-display.md
quality-and-brevity.md
reuse allocations where possible (m-mem-reuse).md
reuse-allocations-where-possible-m-mem-reuse.md
rust code solves rust problems (m-rust-shaped).md
rust-code-solves-rust-problems-m-rust-shaped.md
safety.md
services-are-clone-m-services-clone.md
shrink collections to fit after building (m-shrink-to-fit).md
shrink-collections-to-fit-after-building-m-shrink-to-fit.md
struct-a.md
struct-account.md
struct-b.md
struct-c.md
struct-currency.md
struct-deposit.md
struct-foo.md
struct-foobuilder.md
struct-telemetry-impl-telemetry-fn-ping-self-u32.md
test-utilities-are-feature-gated-m-test-util.md
tests do not assert ground truth (m-tautological-tests).md
tests-do-not-assert-ground-truth-m-tautological-tests.md
the workspace lists and versions all crates (m-crates-in-workspace).md
the-workspace-lists-and-versions-all-crates-m-crates-in-workspace.md
third party items come from hidden `_private` module (m-macro-helpers).md
third-party-items-come-from-hidden-private-module-m-macro-helpers.md
todo.md
trait-dataaccess.md
trait-download.md
type-hnative-const-u8.md
types are send (m-types-send).md
types-are-send-m-types-send.md
universal.md
unsafe implies undefined behavior (m-unsafe-implies-ub).md
unsafe needs reason, should be avoided (m-unsafe).md
unsafe-implies-undefined-behavior-m-unsafe-implies-ub.md
unsafe-needs-reason-should-be-avoided-m-unsafe.md
use a fast hasher where possible (m-fast-hasher).md
use boxed slices and strings for immutable owned sequences (m-box-dst).md
use mimalloc for apps (m-mimalloc-apps).md
use static verification (m-static-verification).md
use structured logging with message templates (m-log-structured).md
use-a-fast-hasher-where-possible-m-fast-hasher.md
use-boxed-slices-and-strings-for-immutable-owned-sequences-m-box-dst.md
use-mimalloc-for-apps-m-mimalloc-apps.md
use-static-verification-m-static-verification.md
use-std-backtrace-backtrace.md
use-std-fmt-display.md
use-std-fmt-formatter.md
use-std-future-future.md
use-std-ops-rangebounds-range.md
use-std-path-path.md
use-std-pin-pin.md
use-std-rc-rc.md
use-std-sync-arc.md
use-std-sync-atomic-atomicusize.md
use-std-sync-atomic-ordering.md
use-std-task-context-poll.md
use-structured-logging-with-message-templates-m-log-structured.md
use-the-proper-type-family-m-strong-types.md
# Rust Engineering Guidelines (Microsoft Pragmatic Rust Guidelines — split sections)

Sections:
- Rust Engineering Guidelines (Microsoft Pragmatic Rust Guidelines — split sections)
- Abstractions Don T Visibly Nest M Simple Abstractions
- Accept `Impl 'Io'` Where Feasible ('Sans Io') (M Impl Io)
- Accept `Impl Asref<>` Where Feasible (M Impl Asref)
- use std::path::Path;
- Accept `Impl Rangebounds<>` Where Feasible (M Impl Rangebounds)
- use std::ops::{RangeBounds, Range};
- Accept Impl Asref Where Feasible M Impl Asref
- use std::path::Path;
- Accept Impl Io Where Feasible Sans Io M Impl Io
- Accept Impl Rangebounds Where Feasible M Impl Rangebounds
- use std::ops::{RangeBounds, Range};
- Add Other Platforms Here Based On Needs
- Add other platforms here based on needs ...
- Additional Lints Are From The Restriction And Nursery Group That Prevent Specific
- Additional lints are from the `restriction` and `nursery` group that prevent specific
- Ai
- AI Guidelines
- All Code Must Be Sound (M Unsound)
- Documentation
