use dioxus::prelude::*;

// ─────────────────────────────────────────────
//  Icon Component Helper & Common Props
// ─────────────────────────────────────────────

#[derive(Props, Clone, PartialEq)]
pub struct IconProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub stroke_width: Option<String>,
}

fn resolve_class(class: &Option<String>, default: &str) -> String {
    class.clone().unwrap_or_else(|| default.to_string())
}

// ─────────────────────────────────────────────
//  Standardized SVG Icons
// ─────────────────────────────────────────────

/// Plus / Add Icon (z. B. für Buttons oder Modals)
#[component]
pub fn PlusIcon(props: IconProps) -> Element {
    let class = resolve_class(&props.class, "w-5 h-5");
    let sw = props.stroke_width.as_deref().unwrap_or("2");
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            stroke_width: "{sw}",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M12 4.5v15m7.5-7.5h-15",
            }
        }
    }
}

/// Close / Cross Icon (z. B. für Schließen-Buttons)
#[component]
pub fn CloseIcon(props: IconProps) -> Element {
    let class = resolve_class(&props.class, "w-5 h-5");
    let sw = props.stroke_width.as_deref().unwrap_or("2");
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            stroke_width: "{sw}",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M6 18L18 6M6 6l12 12",
            }
        }
    }
}

/// Search Magnifying Glass Icon (z. B. für Suchfelder)
#[component]
pub fn SearchIcon(props: IconProps) -> Element {
    let class = resolve_class(&props.class, "w-5 h-5");
    let sw = props.stroke_width.as_deref().unwrap_or("2");
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            stroke_width: "{sw}",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z",
            }
        }
    }
}

/// Calendar Icon (z. B. für Event-Datum oder Termine)
#[component]
pub fn CalendarIcon(props: IconProps) -> Element {
    let class = resolve_class(&props.class, "w-5 h-5");
    let sw = props.stroke_width.as_deref().unwrap_or("2");
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            stroke_width: "{sw}",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 012.25-2.25h13.5A2.25 2.25 0 0121 7.5v11.25m-18 0A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75m-18 0v-7.5A2.25 2.25 0 015.25 9h13.5A2.25 2.25 0 0121 11.25v7.5",
            }
        }
    }
}

/// Clock Icon (z. B. für Bearbeitungszeitpunkt oder Dauer)
#[component]
pub fn ClockIcon(props: IconProps) -> Element {
    let class = resolve_class(&props.class, "w-5 h-5");
    let sw = props.stroke_width.as_deref().unwrap_or("2");
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            stroke_width: "{sw}",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M12 6v6h4.5m4.5 0a9 9 0 11-18 0 9 9 0 0118 0z",
            }
        }
    }
}

/// Pencil / Edit Icon (z. B. für Bearbeiten)
#[component]
pub fn EditIcon(props: IconProps) -> Element {
    let class = resolve_class(&props.class, "w-5 h-5");
    let sw = props.stroke_width.as_deref().unwrap_or("2");
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            stroke_width: "{sw}",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L10.582 16.07a4.5 4.5 0 01-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 011.13-1.897l8.932-8.931zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0115.75 21H5.25A2.25 2.25 0 013 18.75V8.25A2.25 2.25 0 015.25 6H10",
            }
        }
    }
}

/// Check / Checkmark Icon (z. B. für Bestätigung / Erfolgsmeldungen)
#[component]
pub fn CheckIcon(props: IconProps) -> Element {
    let class = resolve_class(&props.class, "w-5 h-5");
    let sw = props.stroke_width.as_deref().unwrap_or("2");
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            stroke_width: "{sw}",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M4.5 12.75l6 6 9-13.5",
            }
        }
    }
}

/// Alert / Warning Triangle Icon (z. B. für Warnungen oder Fehler)
#[component]
pub fn AlertIcon(props: IconProps) -> Element {
    let class = resolve_class(&props.class, "w-5 h-5");
    let sw = props.stroke_width.as_deref().unwrap_or("2");
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            stroke_width: "{sw}",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z",
            }
        }
    }
}

/// Filter / Sort Icon (z. B. für die Toolbars)
#[component]
pub fn FilterIcon(props: IconProps) -> Element {
    let class = resolve_class(&props.class, "w-5 h-5");
    let sw = props.stroke_width.as_deref().unwrap_or("2");
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            stroke_width: "{sw}",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M10.5 6h9.75M10.5 6a1.5 1.5 0 11-3 0m3 0a1.5 1.5 0 10-3 0M3.75 6H7.5m3 12h9.75m-9.75 0a1.5 1.5 0 11-3 0m3 0a1.5 1.5 0 10-3 0M3.75 18H7.5m6-6h6m-6 0a1.5 1.5 0 11-3 0m3 0a1.5 1.5 0 10-3 0M3.75 12h6.75",
            }
        }
    }
}

/// Chevron Down Icon (z. B. für Dropdowns)
#[component]
pub fn ChevronDownIcon(props: IconProps) -> Element {
    let class = resolve_class(&props.class, "w-4 h-4");
    let sw = props.stroke_width.as_deref().unwrap_or("2");
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            stroke_width: "{sw}",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M19.5 8.25l-7.5 7.5-7.5-7.5",
            }
        }
    }
}

/// Chevron Right Icon (z. B. für Breadcrumbs oder Pfeil-Links)
#[component]
pub fn ChevronRightIcon(props: IconProps) -> Element {
    let class = resolve_class(&props.class, "w-4 h-4");
    let sw = props.stroke_width.as_deref().unwrap_or("2");
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            stroke_width: "{sw}",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M8.25 4.5l7.5 7.5-7.5 7.5",
            }
        }
    }
}

/// Loading Spinner Icon (z. B. für Ladeanzeigen)
#[component]
pub fn SpinnerIcon(props: IconProps) -> Element {
    let class = resolve_class(&props.class, "w-5 h-5 animate-spin text-amber-500");
    rsx! {
        svg {
            class: "{class}",
            view_box: "0 0 24 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            circle {
                class: "opacity-25",
                cx: "12",
                cy: "12",
                r: "10",
                stroke: "currentColor",
                stroke_width: "4",
            }
            path {
                class: "opacity-75",
                fill: "currentColor",
                d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z",
            }
        }
    }
}

/// Cloud Storage Icon (z. B. für Cloud-Status Badge)
#[component]
pub fn CloudIcon(props: IconProps) -> Element {
    let class = resolve_class(&props.class, "w-5 h-5");
    let sw = props.stroke_width.as_deref().unwrap_or("2");
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            stroke_width: "{sw}",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M3 15a4 4 0 004 4h9a5 5 0 10-.1-9.999 5.002 5.002 0 00-9.78 2.096A4.001 4.001 0 003 15z",
            }
        }
    }
}

/// Device / Local Storage Icon (z. B. für Lokalen Status Badge)
#[component]
pub fn DeviceIcon(props: IconProps) -> Element {
    let class = resolve_class(&props.class, "w-5 h-5");
    let sw = props.stroke_width.as_deref().unwrap_or("2");
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            stroke_width: "{sw}",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z",
            }
        }
    }
}

/// Upload Icon (z. B. für Cloud Sync)
#[component]
pub fn UploadIcon(props: IconProps) -> Element {
    let class = resolve_class(&props.class, "w-5 h-5");
    let sw = props.stroke_width.as_deref().unwrap_or("2");
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            stroke_width: "{sw}",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M3 16.5v2.25A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75V16.5m-13.5-9L12 3m0 0l4.5 4.5M12 3v13.5",
            }
        }
    }
}

/// Download Icon (z. B. für Lokale Kopie)
#[component]
pub fn DownloadIcon(props: IconProps) -> Element {
    let class = resolve_class(&props.class, "w-5 h-5");
    let sw = props.stroke_width.as_deref().unwrap_or("2");
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            stroke_width: "{sw}",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M3 16.5v2.25A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75V16.5M12 12.75l-4.5-4.5m4.5 4.5l4.5-4.5M12 12.75V3",
            }
        }
    }
}

/// Export Icon (z. B. für .tcc-Datei-Export)
#[component]
pub fn ExportIcon(props: IconProps) -> Element {
    let class = resolve_class(&props.class, "w-5 h-5");
    let sw = props.stroke_width.as_deref().unwrap_or("2");
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            stroke_width: "{sw}",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M12 9.75v6.75m0 0l-3-3m3 3l3-3m-8.25 6h10.5A2.25 2.25 0 0021 19.5V8.25A2.25 2.25 0 0018.75 6H5.25A2.25 2.25 0 003 8.25v11.25A2.25 2.25 0 005.25 21z",
            }
        }
    }
}

/// Trash / Delete Icon
#[component]
pub fn TrashIcon(props: IconProps) -> Element {
    let class = resolve_class(&props.class, "w-5 h-5");
    let sw = props.stroke_width.as_deref().unwrap_or("2");
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            stroke_width: "{sw}",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M14.74 9l-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 01-2.244 2.077H8.084a2.25 2.25 0 01-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 00-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 013.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 00-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 00-7.5 0",
            }
        }
    }
}

/// User Group Icon (z. B. für Team-Größe / Mitglieder)
#[component]
pub fn UserGroupIcon(props: IconProps) -> Element {
    let class = resolve_class(&props.class, "w-5 h-5");
    let sw = props.stroke_width.as_deref().unwrap_or("2");

    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            stroke_width: "{sw}",
            class: "{class}",

            // Center user
            circle {
                cx: "12",
                cy: "8",
                r: "3",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }

            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M6.5 19c0-2.8 2.4-5 5.5-5s5.5 2.2 5.5 5",
            }

            // Left user
            circle {
                cx: "5.5",
                cy: "10",
                r: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }

            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M2.5 18c0-1.8 1.5-3.2 3.5-3.5",
            }

            // Right user
            circle {
                cx: "18.5",
                cy: "10",
                r: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }

            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M21.5 18c0-1.8-1.5-3.2-3.5-3.5",
            }
        }
    }
}

/// Pencil / Edit Icon
#[component]
pub fn PencilIcon(props: IconProps) -> Element {
    let class = resolve_class(&props.class, "w-4 h-4");
    let sw = props.stroke_width.as_deref().unwrap_or("2");
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            stroke_width: "{sw}",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L10.582 16.07a4.5 4.5 0 01-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 011.13-1.897l8.932-8.931zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0115.75 21H5.25A2.25 2.25 0 013 18.75V8.25A2.25 2.25 0 015.25 6H10",
            }
        }
    }
}

/// QR-Code Icon
#[component]
pub fn QrCodeIcon(props: IconProps) -> Element {
    let class = resolve_class(&props.class, "w-4 h-4");
    let sw = props.stroke_width.as_deref().unwrap_or("2");
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            stroke_width: "{sw}",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M3.75 4.875c0-.621.504-1.125 1.125-1.125h4.5c.621 0 1.125.504 1.125 1.125v4.5c0 .621-.504 1.125-1.125 1.125h-4.5A1.125 1.125 0 013.75 9.375v-4.5zM3.75 14.625c0-.621.504-1.125 1.125-1.125h4.5c.621 0 1.125.504 1.125 1.125v4.5c0 .621-.504 1.125-1.125 1.125h-4.5a1.125 1.125 0 01-1.125-1.125v-4.5zM14.625 3.75c-.621 0-1.125.504-1.125 1.125v4.5c0 .621.504 1.125 1.125 1.125h4.5c.621 0 1.125-.504 1.125-1.125v-4.5c0-.621-.504-1.125-1.125-1.125h-4.5zM13.5 13.5h.75v.75h-.75v-.75zM13.5 19.5h.75v.75h-.75v-.75zM19.5 13.5h.75v.75h-.75v-.75zM19.5 19.5h.75v.75h-.75v-.75zM16.5 16.5h.75v.75h-.75v-.75z",
            }
        }
    }
}

 #[component]
pub(crate) fn EndSVG() -> Element {
    rsx!(
        svg {
            class: "w-5 h-5",
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke_width: "2",
            stroke: "#D67229",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M4 2v20m0-18h10l-2 4 2 4H4",
            }
            circle {
                cx: "4",
                cy: "20",
                r: "1",
                fill: "#D67229",
            }
        }
    )
}

#[component]
pub(crate) fn StartSVG() -> Element {
    rsx!(
        svg {
            class: "w-5 h-5",
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke_width: "2",
            stroke: "#D67229",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M12 2l3.09 6.26L22 9.27l-5 4.87L18.18 22 12 18.27 5.82 22 7 14.14l-5-4.87 6.91-1.01L12 2z",
            }
        }
    )
}
