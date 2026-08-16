//! Die einzelne Team-Kachel im Grid der Übersicht.

use crate::side::AddressSVG;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub(super) struct TeamCardProps {
    pub(super) name: String,
    pub(super) address: String,
    pub(super) needs_check: bool,
}

#[component]
pub(super) fn TeamCard(props: TeamCardProps) -> Element {
    let needs_check = props.needs_check;
    let style_0 = if needs_check {
        "bg-amber-100/80 border-amber-300"
    } else {
        "bg-amber-50/70 border-amber-100"
    };
    let style_1 = if needs_check {
        "bg-amber-500"
    } else {
        "bg-amber-400/70"
    };

    rsx! {
        div {
            div { class: "px-4 py-2.5 border-b flex items-center gap-2 {style_0}",
                div { class: "w-1.5 h-4 rounded-full shrink-0 {style_1}" }
                span { class: "text-sm font-semibold text-zinc-800 truncate", "{props.name}" }
                if needs_check {
                    span { class: "ml-auto text-[10px] font-bold px-1.5 py-0.5 rounded-full bg-amber-500 text-white tracking-wide shrink-0",
                        "REVIEW"
                    }
                }
            }
            div { class: "px-4 py-3",
                div { class: "flex items-center gap-1.5",
                    AddressSVG {}
                    p { class: "text-xs text-zinc-500 truncate", "{props.address}" }
                }
            }
        }
    }
}
