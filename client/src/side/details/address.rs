use dioxus::prelude::*;
use web_sys::console;

use crate::{
    address_connector::get_address, async_action, side::{AddressSVG, AsyncAction, InfoSVG}, storage::AddressData, ui::{
        Language, buttons::SecondaryButton, forms::{Input, InputError}, typography::{FieldLabel, Headline3},
    },
};



// ─────────────────────────────────────────────
//  AddressParam
// ─────────────────────────────────────────────

#[derive(PartialEq, Copy)]
pub(crate) struct AddressParam {
    language: Signal<Language>,
    latitude: Signal<String>,
    latitude_error: Signal<String>,
    longitude: Signal<String>,
    longitude_error: Signal<String>,
    address: Signal<String>,
    address_error: Signal<String>,
    general_error: Signal<String>,
}

impl AddressParam {
    pub(crate) fn new(address: &AddressData) -> Self {
        Self::new_with_language(address, use_signal(|| Language::English))
    }

    pub(crate) fn new_with_language(address: &AddressData, language: Signal<Language>) -> Self {
        let lang = *language.read();

        let latitude = use_signal(|| {
            if address.latitude.is_nan() || address.latitude == 0.0 {
                "".to_string()
            } else {
                address.latitude.to_string()
            }
        });
        let latitude_error = use_signal(|| {
            if address.latitude.is_nan() || address.latitude == 0.0 {
                match lang {
                    Language::English => "Invalid latitude!".to_string(),
                    Language::German => "Ungültiger Breitengrad!".to_string(),
                }
            } else {
                "".to_string()
            }
        });

        let longitude = use_signal(|| {
            if address.longitude.is_nan() || address.longitude == 0.0 {
                "".to_string()
            } else {
                address.longitude.to_string()
            }
        });
        let longitude_error = use_signal(|| {
            if address.longitude.is_nan() || address.longitude == 0.0 {
                match lang {
                    Language::English => "Invalid longitude!".to_string(),
                    Language::German => "Ungültiger Längengrad!".to_string(),
                }
            } else {
                "".to_string()
            }
        });

        let address_signal = use_signal(|| {
            if address.address.is_empty() {
                "".to_string()
            } else {
                address.address.clone()
            }
        });
        let address_error_signal = use_signal(|| {
            if address.address.is_empty() {
                match lang {
                    Language::English => "Address cannot be empty!".to_string(),
                    Language::German => "Adresse darf nicht leer sein!".to_string(),
                }
            } else {
                "".to_string()
            }
        });

        Self {
            language,
            latitude,
            latitude_error,
            longitude,
            longitude_error,
            address: address_signal,
            address_error: address_error_signal,
            general_error: use_signal(|| "".to_string()),
        }
    }

    pub(crate) fn default_with_language(language: Signal<Language>) -> Self {
        Self {
            language,
            latitude: use_signal(|| "".to_string()),
            latitude_error: use_signal(|| "".to_string()),
            longitude: use_signal(|| "".to_string()),
            longitude_error: use_signal(|| "".to_string()),
            address: use_signal(|| "".to_string()),
            address_error: use_signal(|| "".to_string()),
            general_error: use_signal(|| "".to_string()),
        }
    }

    pub(crate) fn check_address_data(&self) -> Result<(), String> {
        console::log_1(&"Checking address data...".into());
        let lang = *self.language.read();

        if !check_addr_input(self.address, self.address_error, lang) {
            let msg = match lang {
                Language::English => "Address cannot be empty!",
                Language::German => "Adresse darf nicht leer sein!",
            };
            return Err(msg.to_string());
        }
        if !check_cord_input(self.latitude, self.latitude_error, lang) {
            let msg = match lang {
                Language::English => "Invalid coordinate!",
                Language::German => "Ungültige Koordinate!",
            };
            return Err(msg.to_string());
        }
        if !check_cord_input(self.longitude, self.longitude_error, lang) {
            let msg = match lang {
                Language::English => "Invalid coordinate!",
                Language::German => "Ungültige Koordinate!",
            };
            return Err(msg.to_string());
        }
        Ok(())
    }

    pub(crate) fn get_address_data(&self) -> Result<AddressData, String> {
        let result = self.check_address_data();
        if result.is_err() {
            Err(result.expect_err("Error expected"))
        } else {
            Ok(AddressData {
                address: self.address.read().to_string(),
                latitude: self
                    .latitude
                    .read()
                    .parse::<f64>()
                    .map_err(|e| format!("Expect latitude to be of type f64: {}", e))?,
                longitude: self
                    .longitude
                    .read()
                    .parse::<f64>()
                    .map_err(|e| format!("Expect longitude to be of type f64: {}", e))?,
            })
        }
    }
}

impl Clone for AddressParam {
    fn clone(&self) -> Self {
        Self {
            language: self.language.clone(),
            latitude: self.latitude.clone(),
            latitude_error: self.latitude_error.clone(),
            longitude: self.longitude.clone(),
            longitude_error: self.longitude_error.clone(),
            address: self.address.clone(),
            address_error: self.address_error.clone(),
            general_error: self.general_error.clone(),
        }
    }
}

impl Default for AddressParam {
    fn default() -> Self {
        Self {
            language: use_signal(|| Language::English),
            latitude: use_signal(|| "".to_string()),
            latitude_error: use_signal(|| "".to_string()),
            longitude: use_signal(|| "".to_string()),
            longitude_error: use_signal(|| "".to_string()),
            address: use_signal(|| "".to_string()),
            address_error: use_signal(|| "".to_string()),
            general_error: use_signal(|| "".to_string()),
        }
    }
}

// ─────────────────────────────────────────────
//  Address Root Component
// ─────────────────────────────────────────────

#[component]
pub(crate) fn Address(param: AddressParam) -> Element {
    let tab_signal = use_signal(|| true);
    let auto_param = param.clone();
    let manual_param = param.clone();
    let lang = *param.language.read();

    let headline_text = match lang {
        Language::English => "Address & Location",
        Language::German => "Adresse & Standort",
    };

    rsx!(
        div { class: "space-y-3",
            // Section Header
            div { class: "flex items-center gap-2 text-amber-800",
                AddressSVG {}
                Headline3 { headline: headline_text.to_string() }
            }

            // Tab Bar
            TabBar { tab_signal, language: param.language }

            if *tab_signal.read() {
                AutoAddress { param: auto_param }
            } else {
                ManualAddress { param: manual_param }
            }
        }
    )
}

// ─────────────────────────────────────────────
//  Tab Bar
// ─────────────────────────────────────────────

#[component]
fn TabBar(tab_signal: Signal<bool>, language: Signal<Language>) -> Element {
    let active_cls =
        "px-3.5 py-1.5 text-xs font-semibold rounded-lg bg-amber-500 text-white shadow-xs transition-all cursor-pointer";
    let inactive_cls =
        "px-3.5 py-1.5 text-xs font-medium rounded-lg text-zinc-600 hover:bg-amber-100/50 hover:text-amber-900 transition-all cursor-pointer";

    let lang = *language.read();
    let auto_text = match lang {
        Language::English => "Automatic Search",
        Language::German => "Automatische Suche",
    };
    let manual_text = match lang {
        Language::English => "Manual Coordinates",
        Language::German => "Manuelle Koordinaten",
    };

    rsx!(
        div { class: "flex items-center gap-1.5 p-1 bg-amber-50/60 rounded-xl border border-amber-100/80 mb-3 w-fit",
            button {
                r#type: "button",
                id: "tab-search",
                class: if *tab_signal.read() { active_cls } else { inactive_cls },
                onclick: move |_| {
                    tab_signal.set(true);
                },
                "{auto_text}"
            }
            button {
                r#type: "button",
                id: "tab-coords",
                class: if !*tab_signal.read() { active_cls } else { inactive_cls },
                onclick: move |_| {
                    tab_signal.set(false);
                },
                "{manual_text}"
            }
        }
    )
}

// ─────────────────────────────────────────────
//  Auto Address (Nominatim search)
// ─────────────────────────────────────────────

#[component]
fn AutoAddress(mut param: AddressParam) -> Element {
    let mut is_searching_signal = use_signal(|| false);
    let mut address_search_signal = use_signal(|| "".to_string());
    let address_search_error_signal = use_signal(|| "".to_string());
    let mut address_search_response_error_signal = use_signal(|| "".to_string());

    let lang = *param.language.read();

    let label_text = match lang {
        Language::English => "Search Address",
        Language::German => "Adresse suchen",
    };
    let tooltip_text = match lang {
        Language::English => "The entered address will be forwarded to Nominatim (OpenStreetMap) for location determination.",
        Language::German => "Die eingegebene Adresse wird zur Standortbestimmung an Nominatim (OpenStreetMap) weitergeleitet.",
    };
    let placeholder_text = match lang {
        Language::English => "Street, City, ZIP code",
        Language::German => "Straße, Stadt, PLZ",
    };
    let btn_text = match lang {
        Language::English => "Search Address",
        Language::German => "Adresse suchen",
    };
    let no_address_set_text = match lang {
        Language::English => "No address set",
        Language::German => "Keine Adresse angegeben",
    };

    let search_action: AsyncAction = async_action!({
        console::log_1(&"Start async searching for address!".into());
        let current_lang = *param.language.read();
        if !check_addr_input(address_search_signal, address_search_error_signal, current_lang) {
            is_searching_signal.set(false);
            return;
        }
        let search_address = address_search_signal.read().to_string();
        match get_address(&search_address).await {
            Ok(address) => {
                let addr_data = format!(
                    "{} {}, {} {}",
                    address.address.clone().road.unwrap_or("-".to_string()),
                    address
                        .address
                        .clone()
                        .house_number
                        .unwrap_or("-".to_string()),
                    address.address.clone().postcode.unwrap_or("-".to_string()),
                    address.address.clone().get_city(),
                );
                param.address.set(addr_data);
                param.address_error.set("".to_string());
                param.latitude.set(address.lat.to_string());
                param.latitude_error.set("".to_string());
                param.longitude.set(address.lon.to_string());
                param.longitude_error.set("".to_string());
                address_search_response_error_signal.set("".to_string());
            }
            Err(e) => {
                console::error_1(&format!("Error getting coordinates: {}", e).into());
                let err_msg = match current_lang {
                    Language::English => "No address found!",
                    Language::German => "Keine Adresse gefunden!",
                };
                address_search_response_error_signal.set(err_msg.to_string());
            }
        }
        is_searching_signal.set(false);
        console::log_1(&"Finished searching for address!".into());
    });

    rsx!(
        div { id: "address-search", class: "space-y-3",

            // Label + Privacy Tooltip
            div { class: "flex items-center justify-between",
                FieldLabel { text: label_text.to_string() }
                div { class: "relative group cursor-pointer text-amber-700/70 hover:text-amber-900 transition-colors",
                    InfoSVG {}
                    div { class: "absolute bottom-full right-0 mb-2 hidden group-hover:block \
                                   bg-zinc-800 text-white text-xs rounded-xl \
                                   px-3 py-2 w-64 z-20 shadow-lg leading-relaxed pointer-events-none",
                        "{tooltip_text}"
                    }
                }
            }

            Input {
                place_holer: Some(placeholder_text.to_string()),
                value: address_search_signal.read().clone(),
                is_error: !address_search_error_signal.read().is_empty(),
                oninput: move |e: FormEvent| {
                    let address = e.value();
                    address_search_signal.set(address);
                    let _ = check_addr_input(
                        address_search_signal,
                        address_search_error_signal,
                        lang,
                    );
                },
            }

            InputError { error: address_search_error_signal.read().clone() }

            SecondaryButton { text: btn_text.to_string(), action: search_action }

            // Result / Status Row
            div { class: "pt-1 flex items-start gap-2 text-xs min-h-[1.5rem]",

                if !address_search_response_error_signal.read().is_empty() {
                    InputError { error: address_search_response_error_signal.read().clone() }
                } else if !param.address.read().is_empty() {
                    svg {
                        class: "w-4 h-4 shrink-0 mt-0.5 text-emerald-500",
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke_width: "2",
                        stroke: "currentColor",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z",
                        }
                    }
                    span { class: "text-zinc-700 font-medium leading-snug", "{param.address}" }
                } else if !param.address_error.read().is_empty() {
                    InputError { error: param.address_error.read().clone() }
                } else {
                    svg {
                        class: "w-4 h-4 shrink-0 mt-0.5 text-zinc-300",
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke_width: "2",
                        stroke: "currentColor",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M12 4.5v15m7.5-7.5h-15",
                        }
                    }
                    span { class: "text-zinc-400 font-medium", "{no_address_set_text}" }
                }
            }
        }
    )
}

// ─────────────────────────────────────────────
//  Manual Address (coordinate entry)
// ─────────────────────────────────────────────

#[component]
fn ManualAddress(mut param: AddressParam) -> Element {
    let lang = *param.language.read();

    let lat_label = match lang {
        Language::English => "Latitude",
        Language::German => "Breitengrad",
    };
    let lat_placeholder = match lang {
        Language::English => "e.g. 50.1127197",
        Language::German => "z.B. 50.1127197",
    };

    let lon_label = match lang {
        Language::English => "Longitude",
        Language::German => "Längengrad",
    };
    let lon_placeholder = match lang {
        Language::English => "e.g. 8.682092",
        Language::German => "z.B. 8.682092",
    };

    let addr_label = match lang {
        Language::English => "Address Label",
        Language::German => "Adressenbezeichnung",
    };
    let addr_placeholder = match lang {
        Language::English => "e.g. Main Street 1, 12345 City",
        Language::German => "z.B. Hauptstraße 1, 12345 Stadt",
    };

    rsx!(
        div { id: "coordinates", class: "space-y-3",

            div {
                FieldLabel { text: lat_label.to_string() }
                Input {
                    place_holer: Some(lat_placeholder.to_string()),
                    value: param.latitude.read().clone(),
                    is_error: !param.latitude_error.read().is_empty(),
                    oninput: move |e: FormEvent| {
                        param.latitude.set(e.value());
                        let _ = check_cord_input(param.latitude, param.latitude_error, lang);
                    },
                }
                InputError { error: param.latitude_error.read().clone() }
            }

            div {
                FieldLabel { text: lon_label.to_string() }
                Input {
                    place_holer: Some(lon_placeholder.to_string()),
                    value: param.longitude.read().clone(),
                    is_error: !param.longitude_error.read().is_empty(),
                    oninput: move |e: FormEvent| {
                        param.longitude.set(e.value());
                        let _ = check_cord_input(param.longitude, param.longitude_error, lang);
                    },
                }
                InputError { error: param.longitude_error.read().clone() }
            }

            div {
                FieldLabel { text: addr_label.to_string() }
                Input {
                    place_holer: Some(addr_placeholder.to_string()),
                    value: param.address.read().clone(),
                    is_error: !param.address_error.read().is_empty(),
                    oninput: move |e: FormEvent| {
                        param.address.set(e.value());
                        let _ = check_addr_input(param.address, param.address_error, lang);
                    },
                }
                InputError { error: param.address_error.read().clone() }
            }
        }
    )
}

// ─────────────────────────────────────────────
//  Validation Helpers
// ─────────────────────────────────────────────

fn check_addr_input(
    input_signal: Signal<String>,
    mut error_signal: Signal<String>,
    lang: Language,
) -> bool {
    if input_signal.read().trim().is_empty() {
        let msg = match lang {
            Language::English => "Address cannot be empty!",
            Language::German => "Adresse darf nicht leer sein!",
        };
        error_signal.set(msg.to_string());
        false
    } else {
        error_signal.set("".to_string());
        true
    }
}

fn check_cord_input(
    input_signal: Signal<String>,
    mut error_signal: Signal<String>,
    lang: Language,
) -> bool {
    let read_value = input_signal.read();
    match read_value.parse::<f64>() {
        Ok(_) => {
            error_signal.set("".to_string());
            true
        }
        Err(_) => {
            let msg = match lang {
                Language::English => "Invalid coordinate!",
                Language::German => "Ungültige Koordinate!",
            };
            error_signal.set(msg.to_string());
            false
        }
    }
}