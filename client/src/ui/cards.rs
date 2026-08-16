use dioxus::prelude::*;
use crate::ui::forms::SearchInput;

#[derive(Props, Clone, PartialEq)]
pub struct BaseCardProps {
    #[props(default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn BaseCard(props: BaseCardProps) -> Element {
    rsx! {
        div { class: "bg-white rounded-2xl border border-amber-100/80 shadow-xs hover:border-amber-300 hover:shadow-md transition-all duration-200 overflow-hidden flex flex-col justify-between h-full {props.class}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardHeaderProps {
    pub title: String,
    #[props(default)]
    pub subtitle: Option<String>,
    #[props(default)]
    pub action: Option<Element>,
    #[props(default)]
    pub class: String,
}

#[component]
pub fn CardHeader(props: CardHeaderProps) -> Element {
    rsx! {
        div { class: "px-4 py-3 bg-amber-50/60 border-b border-amber-100/70 flex items-center justify-between gap-2.5 {props.class}",
            div { class: "flex items-center gap-2.5 min-w-0",
                div { class: "w-1.5 h-4 rounded-full bg-amber-400 shrink-0" }
                div { class: "min-w-0",
                    h3 { class: "text-sm font-semibold text-zinc-800 truncate", "{props.title}" }
                    if let Some(sub) = &props.subtitle {
                        p { class: "text-xs text-zinc-500 truncate", "{sub}" }
                    }
                }
            }
            if let Some(act) = props.action {
                div { class: "shrink-0", {act} }
            }
        }
    }
}

// ─────────────────────────────────────────────
//  Search & Filter Card Component
// ─────────────────────────────────────────────

#[derive(Props, Clone, PartialEq)]
pub struct SearchFilterCardProps<T: Clone + PartialEq + 'static> {
    pub search_value: String,
    #[props(default)]
    pub search_placeholder: Option<String>,
    pub on_search_change: EventHandler<String>,

    pub sort_value: T,
    pub sort_options: Vec<(T, String)>,
    pub on_sort_change: EventHandler<T>,

    #[props(default = true)]
    pub use_card_wrapper: bool,
    #[props(default)]
    pub class: String,
}

#[component]
pub fn SearchFilterCard<T: Clone + PartialEq + 'static>(props: SearchFilterCardProps<T>) -> Element {
    let selected_index = props
        .sort_options
        .iter()
        .position(|(val, _)| *val == props.sort_value)
        .unwrap_or(0);

let inner_content = rsx! {
    div { class: "flex flex-col sm:flex-row sm:items-center justify-between gap-3 sm:gap-4 p-3 sm:p-4 w-full bg-white/60 backdrop-blur-md border border-amber-900/10 rounded-2xl shadow-xs",

        // Suchfeld nimmt auf Desktop flexibel den verfügbaren Platz ein
        div { class: "w-full sm:flex-1 sm:max-w-md",
            SearchInput {
                placeholder: props.search_placeholder,
                value: props.search_value.clone(),
                oninput: move |val| props.on_search_change.call(val),
            }
        }

        // Sortierung: Auf Mobile volle Breite/Rechtsbündig, auf Desktop kompakt rechts
        div { class: "flex items-center gap-2.5 w-full sm:w-auto shrink-0 justify-end",
            label { class: "text-xs font-bold uppercase tracking-wider text-amber-900/70 hidden sm:inline select-none shrink-0",
                "Sort:"
            }

            div { class: "relative w-full sm:w-auto flex items-center min-w-[160px]",
                // Sort-Icon (Links im Input eingebettet)
                svg {
                    class: "absolute left-3 w-4 h-4 text-amber-700 pointer-events-none z-10 shrink-0",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    view_box: "0 0 24 24",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: "M3 4h13M3 8h9m-9 4h6m4 0l4-4m0 0l4 4m-4-4v12",
                    }
                }

                select {
                    id: "sort-select-input",
                    aria_label: "Choose sorting option",
                    // pl-9 für das linke Icon, pr-9 um Platz für den Pfeil rechts zu garantieren (verhindert Überlappung)
                    class: "w-full appearance-none bg-amber-50/80 hover:bg-amber-100/70 focus:bg-white text-xs sm:text-sm font-semibold text-amber-950 border border-amber-200/90 hover:border-amber-300 rounded-xl pl-9 pr-9 py-2.5 sm:py-2 transition-all shadow-2xs cursor-pointer focus:outline-none focus:ring-2 focus:ring-amber-500/40 focus:border-amber-500",
                    value: "{selected_index}",
                    onchange: move |e: Event<FormData>| {
                        if let Ok(idx) = e.value().parse::<usize>() {
                            if let Some((val, _)) = props.sort_options.get(idx) {
                                props.on_sort_change.call(val.clone());
                            }
                        }
                    },
                    for (idx , (_ , label)) in props.sort_options.iter().enumerate() {
                        option {
                            key: "{idx}",
                            value: "{idx}",
                            selected: idx == selected_index,
                            "{label}"
                        }
                    }
                }

                // Custom Dropdown Arrow Icon (Rechts im Input)
                div { class: "absolute right-3 pointer-events-none text-amber-700/80 flex items-center z-10",
                    svg {
                        class: "w-4 h-4",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M19 9l-7 7-7-7",
                        }
                    }
                }
            }
        }
    }
};

    if props.use_card_wrapper {
        rsx! {
            div { class: "bg-white/80 backdrop-blur-xs rounded-2xl border border-amber-100/80 shadow-xs w-full h-auto {props.class}",
                {inner_content}
            }
        }
    } else {
        rsx! {
            div { class: "w-full h-auto {props.class}", {inner_content} }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct InfoCardProps {
    pub title: String,
    pub lines: Vec<String>,
    #[props(default)]
    pub class: String,
}

/// Standardisierte Informationskarte für Hinweis- und Statustexte
#[component]
pub fn InfoCard(props: InfoCardProps) -> Element {
    rsx! {
        BaseCard { class: props.class,
            CardHeader { title: props.title }
            div { class: "p-5 space-y-3 text-sm text-zinc-600 leading-relaxed",
                for line in props.lines {
                    p { "{line}" }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct StatBoxProps {
    pub label: String,
    pub value: String,
    #[props(default)]
    pub alert: bool,
}

#[component]
pub fn StatBox(props: StatBoxProps) -> Element {
    let wrapper_class = if props.alert {
        "p-4 rounded-xl border border-red-200 bg-red-50"
    } else {
        "p-4 rounded-xl border border-amber-100 bg-amber-50/50"
    };
    rsx! {
        div { class: wrapper_class,
            p { class: "text-[11px] font-semibold uppercase tracking-[0.12em] text-zinc-400 mb-1",
                "{props.label}"
            }
            p { class: "text-2xl font-bold text-[#C66741]", "{props.value}" }
        }
    }
}