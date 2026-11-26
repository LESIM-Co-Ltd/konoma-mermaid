//! Reading mermaid's state-diagram language — stage 2 of konoma's own renderer.
//!
//! The whole of this module answers one question: *what did the author write?* It produces a
//! [`StateDiagram`] — states, transitions, and which state is inside which — and stops there.
//! No text is measured, no box is sized, no SVG is emitted; [`render::state`] does that, and it
//! does it with stage 1's layout rather than one of its own
//! (`docs/FEATURE-MERMAID-RENDERER.md` §7: "段 1 のレイアウトを流用").
//!
//! ```text
//! let d = parse("stateDiagram-v2\n  [*] --> Idle\n  Idle --> [*]").unwrap();
//! d.states.len()      == 3     // root_start, Idle, root_end
//! d.transitions.len() == 2
//! ```
//! (konoma has no library target, so this is an illustration rather than a doctest; the same
//! case is asserted for real in [`tests`].)
//!
//! # Layout of the module
//!
//! | file | what it owns |
//! |---|---|
//! | [`parser`] | the grammar, plus the three resolutions mermaid does after parsing: `[*]`, `--`, notes |
//! | [`model`] | the intermediate representation the parser produces |
//!
//! Preprocessing is *not* here: a state diagram goes through exactly the same five rewrites a
//! flowchart does (`flowchart::preprocess`), because they are mermaid's `preprocessDiagram()`
//! and it runs before any diagram's parser. The one thing state diagrams add on top — `#` as a
//! second comment marker, and comments at the end of a statement rather than only on a line of
//! their own — lives in [`parser::strip_comment`], because it is a rule of *this* grammar.
//!
//! # What it does not do
//!
//! `classDef`, `class`, `style`, `click`, `scale`, `hide empty description`, `:::`, `%%` and `#`
//! comments, front matter, `accTitle` and `accDescr` in both forms, and the floating
//! `note "text" as id` are all *recognised* — which is the point, since a line-based parser that
//! did not recognise them would turn `classDef hot fill:#f9f` into a box (§2-3). None of them is
//! drawn. Neither is a `direction` written inside a block; see [`parser`] for why.

// Items and re-exports of this module that only the tests use are allowed to go unreported as
// dead code or unused imports.
#![allow(dead_code, unused_imports)]

pub mod model;
pub mod parser;

pub use model::{Direction, Kind, NotePosition, ParseError, State, StateDiagram, Transition};
pub use parser::{is_state_diagram, parse};

#[cfg(test)]
mod tests;
