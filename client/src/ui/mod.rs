pub mod cards;
pub mod dialogs;
pub mod forms;
pub mod tokens;
pub mod buttons;
pub mod typography;
pub mod icons;

// ─────────────────────────────────────────────
//  Language Enum
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum Language {
    #[default]
    English,
    German,
}