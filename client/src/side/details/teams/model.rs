//! Kleine, von mehreren Geschwister-Modulen gemeinsam genutzte Typen:
//! die Sortieroptionen für die Team-Übersicht und der Zustand des
//! aktuell geöffneten Popups/Dialogs.

use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TeamSortOption {
    NameAsc,
    NameDesc,
    MembersDesc,
}

pub(super) fn tab_cls(active: bool) -> &'static str {
    if active {
        "px-4 py-2 text-sm font-semibold text-[#C66741] border-b-2 border-[#C66741] -mb-px"
    } else {
        "px-4 py-2 text-sm font-medium text-zinc-500 hover:text-[#C66741] transition-colors"
    }
}

pub(super) enum PopUpWindow {
    None,
    AddTeam,
    // Hält jetzt nur noch die Team-ID statt eines vollständigen
    // TeamData-Snapshots – die tatsächlichen Daten werden beim Rendern
    // frisch aus `team_list` abgeleitet (Single Source of Truth).
    EditTeam(Uuid),
    // Trägt keine Config-Daten mehr im Enum – wird beim Rendern frisch
    // aus der `share_config`-Resource abgeleitet.
    Share,
}
