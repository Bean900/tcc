use dioxus::prelude::*;
use uuid::Uuid;

use crate::side::Headline1;

#[component]
pub fn Calculate(cook_and_run_id: Uuid) -> Element {
    rsx!(
        div { class: "p-6",
            Headline1 { headline: "Calculation" }
            // Here we would add the actual calculation UI components, such as settings, plans, and preview.
            // For now, we can just display a placeholder.
            div { class: "mt-4 text-gray-500",
                "This is where the calculation settings, plans, and preview will be displayed."
            }
        }
    )
}

#[component]
fn CalculateSettings(cook_and_run_id: Uuid) -> Element {
    //Titel,
    // Beschreibung
    //sprache kann eingestellt werden, z.B. deutsch oder englisch
    rsx!(
        div { class: "p-6",
            // Here we would add the actual calculation UI components, such as settings, plans, and preview.
            // For now, we can just display a placeholder.
            div { class: "mt-4 text-gray-500",
                "This is where the calculation settings, plans, and preview will be displayed."
            }
        }
    )
}

#[component]
fn CalculatePlans(cook_and_run_id: Uuid) -> Element {
    //Liste der Teams um Laufzettel einzeln zu betrachten
    rsx!(
        div { class: "p-6",
            // Here we would add the actual calculation UI components, such as settings, plans, and preview.
            // For now, we can just display a placeholder.
            div { class: "mt-4 text-gray-500",
                "This is where the calculation settings, plans, and preview will be displayed."
            }
        }
    )
}

#[component]
fn CalculatePreview(cook_and_run_id: Uuid) -> Element {
    // Darstellen des Laufzettels
    rsx!(
        div { class: "p-6",
            // Here we would add the actual calculation UI components, such as settings, plans, and preview.
            // For now, we can just display a placeholder.
            div { class: "mt-4 text-gray-500",
                "This is where the calculation settings, plans, and preview will be displayed."
            }
        }
    )
}
