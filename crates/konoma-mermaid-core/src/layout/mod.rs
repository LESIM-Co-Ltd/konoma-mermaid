// Vendored from the `dagre` crate v0.1.1 (https://github.com/kookyleo/dagre-rs).
// Copyright (c) kookyleo <kookyleo@gmail.com>. Licensed under the Apache License,
// Version 2.0 -- full text in `src/layout/LICENSE-APACHE`.
// Modified by the konoma authors; every change is listed in
// `src/layout/PROVENANCE.md`.

//! Sugiyama-style hierarchical graph layout, vendored from the `dagre` crate.
//!
//! # Why the code lives in `src/` instead of being a dependency
//!
//! konoma's own mermaid renderer needs a layout engine it can *fix*. A crates.io
//! dependency cannot be patched in a published crate (`[patch.crates-io]` is stripped
//! by `cargo publish`), so the source is carried here and ships inside the
//! `konoma-mermaid-core` crate.
//!
//! # Why it is not rewritten
//!
//! `cross-validate/reference_data.json` holds the output of the real
//! `@dagrejs/dagre` JavaScript implementation for 30 graphs. That is the only
//! *external* truth this renderer has: it is not konoma checking konoma. Rewriting
//! the port into idiomatic Rust would silently invalidate that check, so the code is
//! kept as close to upstream as the language allows — see `PROVENANCE.md` for the
//! exact list of changes and `cross_validate.rs` for the test that consumes the data.
//!
//! # Provenance and license
//!
//! Vendored from `dagre` 0.1.1 (<https://github.com/kookyleo/dagre-rs>, archived),
//! Copyright (c) kookyleo <kookyleo@gmail.com>, licensed under the Apache License,
//! Version 2.0. The full license text is in `LICENSE-APACHE` next to this file; the
//! modifications konoma made are listed in `PROVENANCE.md`. konoma itself is MIT —
//! this subtree stays Apache-2.0. `dagre` is in turn a port of dagre.js and graphlib
//! (MIT); their notices are in `THIRD-PARTY-NOTICES.md` at the crate root.

// This subtree is deliberately *not* brought in line with konoma's own style: every
// edit costs us a piece of the dagre.js cross-validation guarantee above. The lints
// below are therefore allowed for the whole vendored subtree rather than fixed.
//
// * `dead_code`, `unused_imports` — the mermaid renderer uses only part of upstream's
//   public API; the unused rest is kept rather than cut.
// * `clippy::collapsible_if` — upstream is edition 2024 and uses let-chains
//   (`if let Some(a) = x && let Some(b) = y`). konoma is edition 2021, where those do
//   not parse, so each chain was mechanically nested. Collapsing them back is exactly
//   what clippy asks for and exactly what the edition forbids.
// * `clippy::manual_filter`, `clippy::unnecessary_sort_by` — pre-existing upstream
//   style that a newer clippy flags. Upstream is archived and will not fix it, and
//   touching it here buys nothing.
// * `clippy::module_inception` — upstream's crate root has `graph` and `layout`
//   modules. Vendoring the crate under `crate::layout` therefore produces
//   `layout::layout`. Keeping upstream's file paths is what makes a diff against a
//   future dagre release readable, so the nesting stays.
#![allow(
    dead_code,
    unused_imports,
    clippy::collapsible_if,
    clippy::module_inception,
    clippy::manual_filter,
    clippy::unnecessary_sort_by
)]

pub mod graph;
pub mod layout;
mod util;

pub use layout::intersect;
pub use layout::layout;
pub use layout::types::{
    Acyclicer, Align, EdgeLabel, GraphLabel, LayoutOptions, NodeLabel, Point, RankAlign, RankDir,
    Ranker,
};

#[cfg(test)]
mod cross_validate;
