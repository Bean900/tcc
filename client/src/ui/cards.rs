use dioxus::prelude::*;

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