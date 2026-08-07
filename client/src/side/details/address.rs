use dioxus::prelude::*;
use web_sys::console;

use crate::{
    address_connector::get_address,
    async_action,
    side::{AddressSVG, AsyncAction, InfoSVG},
    storage::AddressData,
    ui::{
        buttons::SecondaryButton,
        forms::{Input, InputError},
        typography::{FieldLabel, Headline3},
    },
};

// ─────────────────────────────────────────────
//  AddressParam (logic & struct unchanged)
// ─────────────────────────────────────────────

#[derive(PartialEq, Copy)]
pub(crate) struct AddressParam {
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
        let latitude = use_signal(|| {
            if address.latitude.is_nan() || address.latitude == 0.0 {
                "".to_string()
            } else {
                address.latitude.to_string()
            }
        });
        let latitude_error = use_signal(|| {
            if address.latitude.is_nan() || address.latitude == 0.0 {
                "Invalid latitude!".to_string()
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
                "Invalid longitude!".to_string()
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
                "Address cannot be empty!".to_string()
            } else {
                "".to_string()
            }
        });

        Self {
            latitude,
            latitude_error,
            longitude,
            longitude_error,
            address: address_signal,
            address_error: address_error_signal,
            general_error: use_signal(|| "".to_string()),
        }
    }

    pub(crate) fn check_address_data(&self) -> Result<(), String> {
        console::log_1(&"Checking address data...".into());
        if !check_addr_input(self.address, self.address_error) {
            return Err("Address cannot be empty!".to_string());
        }
        if !check_cord_input(self.latitude, self.latitude_error) {
            return Err("Invalid coordinate!".to_string());
        }
        if !check_cord_input(self.longitude, self.longitude_error) {
            return Err("Invalid coordinate!".to_string());
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

    rsx!(
        div { class: "space-y-3",
            // Section Header
            div { class: "flex items-center gap-2 text-amber-800",
                AddressSVG {}
                Headline3 { headline: "Address & Location".to_string() }
            }

            // Tab Bar
            TabBar { tab_signal }

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
fn TabBar(tab_signal: Signal<bool>) -> Element {
    let active_cls =
        "px-3.5 py-1.5 text-xs font-semibold rounded-lg bg-amber-500 text-white shadow-xs transition-all cursor-pointer";
    let inactive_cls =
        "px-3.5 py-1.5 text-xs font-medium rounded-lg text-zinc-600 hover:bg-amber-100/50 hover:text-amber-900 transition-all cursor-pointer";

    rsx!(
        div { class: "flex items-center gap-1.5 p-1 bg-amber-50/60 rounded-xl border border-amber-100/80 mb-3 w-fit",
            button {
                r#type: "button",
                id: "tab-search",
                class: if *tab_signal.read() { active_cls } else { inactive_cls },
                onclick: move |_| {
                    tab_signal.set(true);
                },
                "Automatic Search"
            }
            button {
                r#type: "button",
                id: "tab-coords",
                class: if !*tab_signal.read() { active_cls } else { inactive_cls },
                onclick: move |_| {
                    tab_signal.set(false);
                },
                "Manual Coordinates"
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

    let search_action: AsyncAction = async_action!({
        console::log_1(&"Start async searching for address!".into());
        if !check_addr_input(address_search_signal, address_search_error_signal) {
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
                address_search_response_error_signal.set("No address found!".to_string());
            }
        }
        is_searching_signal.set(false);
        console::log_1(&"Finished searching for address!".into());
    });

    rsx!(
        div { id: "address-search", class: "space-y-3",

            // Label + Privacy Tooltip
            div { class: "flex items-center justify-between",
                FieldLabel { text: "Search Address".to_string() }
                div { class: "relative group cursor-pointer text-amber-700/70 hover:text-amber-900 transition-colors",
                    InfoSVG {}
                    div { class: "absolute bottom-full right-0 mb-2 hidden group-hover:block \
                                   bg-zinc-800 text-white text-xs rounded-xl \
                                   px-3 py-2 w-64 z-20 shadow-lg leading-relaxed pointer-events-none",
                        "The entered address will be forwarded to Nominatim (OpenStreetMap) for location determination."
                    }
                }
            }

            Input {
                place_holer: Some("Street, City, ZIP code".to_string()),
                value: address_search_signal.read().clone(),
                is_error: !address_search_error_signal.read().is_empty(),
                oninput: move |e: FormEvent| {
                    let address = e.value();
                    address_search_signal.set(address);
                    let _ = check_addr_input(address_search_signal, address_search_error_signal);
                },
            }

            InputError { error: address_search_error_signal.read().clone() }

            SecondaryButton { text: "Search Address".to_string(), action: search_action }

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
                    span { class: "text-zinc-400 font-medium", "No address set" }
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
    rsx!(
        div { id: "coordinates", class: "space-y-3",

            div {
                FieldLabel { text: "Latitude".to_string() }
                Input {
                    place_holer: Some("e.g. 50.1127197".to_string()),
                    value: param.latitude.read().clone(),
                    is_error: !param.latitude_error.read().is_empty(),
                    oninput: move |e: FormEvent| {
                        param.latitude.set(e.value());
                        let _ = check_cord_input(param.latitude, param.latitude_error);
                    },
                }
                InputError { error: param.latitude_error.read().clone() }
            }

            div {
                FieldLabel { text: "Longitude".to_string() }
                Input {
                    place_holer: Some("e.g. 8.682092".to_string()),
                    value: param.longitude.read().clone(),
                    is_error: !param.longitude_error.read().is_empty(),
                    oninput: move |e: FormEvent| {
                        param.longitude.set(e.value());
                        let _ = check_cord_input(param.longitude, param.longitude_error);
                    },
                }
                InputError { error: param.longitude_error.read().clone() }
            }

            div {
                FieldLabel { text: "Address Label".to_string() }
                Input {
                    place_holer: Some("e.g. Main Street 1, 12345 City".to_string()),
                    value: param.address.read().clone(),
                    is_error: !param.address_error.read().is_empty(),
                    oninput: move |e: FormEvent| {
                        param.address.set(e.value());
                        let _ = check_addr_input(param.address, param.address_error);
                    },
                }
                InputError { error: param.address_error.read().clone() }
            }
        }
    )
}

// ─────────────────────────────────────────────
//  Validation Helpers (logic unchanged)
// ─────────────────────────────────────────────

fn check_addr_input(input_signal: Signal<String>, mut error_signal: Signal<String>) -> bool {
    if input_signal.read().trim().is_empty() {
        error_signal.set("Address cannot be empty!".to_string());
        false
    } else {
        error_signal.set("".to_string());
        true
    }
}

fn check_cord_input(input_signal: Signal<String>, mut error_signal: Signal<String>) -> bool {
    let read_value = input_signal.read();
    match read_value.parse::<f64>() {
        Ok(_) => {
            error_signal.set("".to_string());
            true
        }
        Err(_) => {
            error_signal.set("Invalid coordinate!".to_string());
            false
        }
    }
}