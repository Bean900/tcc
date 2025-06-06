use crate::address_connector::get_address;
use dioxus::prelude::*;
use web_sys::console;

use crate::{
    side::{BlueButton, Input, InputError},
    storage::AddressData,
};

#[derive(PartialEq, Clone, Copy)]
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
        let latitude;
        let latitude_error;
        if address.latitude.is_nan() || address.latitude == 0.0 {
            latitude = use_signal(|| "".to_string());
            latitude_error = use_signal(|| "Invalid latitude!".to_string());
        } else {
            latitude = use_signal(|| address.latitude.to_string());
            latitude_error = use_signal(|| "".to_string());
        }

        let longitude;
        let longitude_error;
        if address.longitude.is_nan() || address.longitude == 0.0 {
            longitude = use_signal(|| "".to_string());
            longitude_error = use_signal(|| "Invalid longitude!".to_string());
        } else {
            longitude = use_signal(|| address.longitude.to_string());
            longitude_error = use_signal(|| "".to_string());
        }

        let address_signal;
        let address_error_signal;
        if address.address.is_empty() {
            address_signal = use_signal(|| "".to_string());
            address_error_signal = use_signal(|| "Address cannot be empty!".to_string());
        } else {
            address_signal = use_signal(|| address.address.clone());
            address_error_signal = use_signal(|| "".to_string());
        }

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

    pub(crate) fn default() -> Self {
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

    pub(crate) fn get_data_signals(&self) -> (Signal<String>, Signal<String>, Signal<String>) {
        (self.latitude, self.longitude, self.address)
    }

    pub(crate) fn check_address_data(&self) -> Result<(), String> {
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
                    .expect("Expect latitude to be of type f64"),
                longitude: self
                    .longitude
                    .read()
                    .parse::<f64>()
                    .expect("Expect longitude to be of type f64"),
            })
        }
    }
}

#[component]
pub(crate) fn Address(param: AddressParam) -> Element {
    let tab_signal = use_signal(|| true);
    let auto_param = param.clone();
    let manual_param = param.clone();

    rsx!(
        label { class: "block font-semibold text-gray-700 mb-2", "Address" }

        TabBar { tab_signal }

        if *tab_signal.read() {
            AutoAddress { param: auto_param }
        } else {
            ManualAddress { param: manual_param }
        }
    )
}

#[component]
fn TabBar(tab_signal: Signal<bool>) -> Element {
    rsx!(
        div { class: "flex border-b border-gray-300 mb-4",
            button {
                r#type: "button",
                onclick: move |_| {
                    tab_signal.set(true);
                },
                id: "tab-search",
                class: if *tab_signal.read() { "px-4 py-2 font-semibold text-sm text-blue-600 border-b-2 border-blue-600" } else { "px-4 py-2 font-semibold text-sm text-gray-600 hover:text-blue-600" },
                "Automatic"
            }
            button {
                r#type: "button",
                onclick: move |_| {
                    tab_signal.set(false);
                },
                id: "tab-coords",
                class: if !*tab_signal.read() { "px-4 py-2 font-semibold text-sm text-blue-600 border-b-2 border-blue-600" } else { "px-4 py-2 font-semibold text-sm text-gray-600 hover:text-blue-600" },
                "Manual"
            }
        }
    )
}

#[component]
fn AutoAddress(mut param: AddressParam) -> Element {
    let mut address_search_signal = use_signal(|| "".to_string());
    let address_search_error_signal = use_signal(|| "".to_string());
    let mut address_search_response_error_signal = use_signal(|| "".to_string());
    rsx!(
        // Search Address
        div { id: "address-search",
            div { class: "flex items-center justify-between mb-2",
                label { class: "block font-semibold text-gray-700", "Search Address" }
                div { class: "flex items-center text-sm text-gray-600",
                    svg {
                        class: "w-4 h-4 mr-1 text-blue-500",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M13 16h-1v-4h-1m1-4h.01M12 18a6 6 0 100-12 6 6 0 000 12z",
                        }
                    }
                    span { class: "relative group cursor-pointer",
                        "Info"
                        span { class: "absolute bottom-full left-1/2 -translate-x-1/2 mb-1 hidden group-hover:block bg-gray-700 text-white text-xs rounded px-2 py-1 w-max max-w-xs z-10 shadow-md",
                            "The entered address will be forwarded to Nominatim (OpenStreetMap) for location determination."
                        }
                    }
                }
            }

            Input {
                place_holer: Some("Street, City, ZIP code".to_string()),
                value: address_search_signal.clone(),
                is_error: !address_search_error_signal.read().is_empty(),
                oninput: move |e: Event<FormData>| {
                    let address = e.value();
                    address_search_signal.set(address);
                    let _ = check_addr_input(address_search_signal, address_search_error_signal);
                },
            }

            InputError { error: address_search_error_signal.read() }

            BlueButton {
                text: "Search".to_string(),
                onclick: move |_| {
                    async move {
                        if !check_addr_input(address_search_signal, address_search_error_signal) {
                            return;
                        }
                        let search_address = address_search_signal.read().to_string();
                        let result = get_address(&search_address).await;
                        if result.is_err() {
                            console::error_1(
                                &format!(
                                    "Error getting coordinates: {}",
                                    result.err().expect("Expected error"),
                                )
                                    .into(),
                            );
                            address_search_response_error_signal
                                .set("No address found!".to_string());
                        } else {
                            let address = result.expect("Expected coordinates");
                            param
                                .address
                                .set(
                                    format!(
                                        "{} {}, {}",
                                        address.address.road.unwrap_or("-".to_string()),
                                        address.address.house_number.unwrap_or("-".to_string()),
                                        address.address.postcode.unwrap_or("-".to_string()),
                                    ),
                                );
                            param.address_error.set("".to_string());
                            param.latitude.set(address.lat.to_string());
                            param.latitude_error.set("".to_string());
                            param.longitude.set(address.lon.to_string());
                            param.longitude_error.set("".to_string());
                            address_search_response_error_signal.set("".to_string());
                        }
                    }
                },
            }

            // Show Found Address
            p { class: "mt-2 text-sm text-gray-700",

                if !address_search_response_error_signal.read().is_empty() {
                    span {
                        InputError { error: address_search_response_error_signal.read() }
                    }
                } else if !param.address.read().is_empty() {
                    span {
                        svg {
                            class: "w-5 h-5 text-green-600 inline-block",
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke_width: "1.5",
                            stroke: "currentColor",

                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                d: "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z",
                            }
                        }
                        "{param.address}"
                    }
                } else {
                    span {
                        svg {
                            class: "w-5 h-5 text-gray-600 inline-block",
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke_width: "1.5",
                            stroke: "currentColor",

                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                d: "M12 4.5v15m7.5-7.5h-15",
                            }
                        }
                        "No address set"
                    }
                }
            }
        }
    )
}

fn check_addr_input(input_signal: Signal<String>, mut error_signal: Signal<String>) -> bool {
    if input_signal.read().trim().is_empty() {
        error_signal.set("Address cannot be empty!".to_string());
        false
    } else {
        error_signal.set("".to_string());
        true
    }
}

#[component]
fn ManualAddress(mut param: AddressParam) -> Element {
    rsx!(
        div { id: "coordinates",

            label { class: "block font-semibold text-gray-700 mb-1", "Latitude" }

            Input {
                place_holer: Some("e.g. 50.1127197".to_string()),
                value: param.latitude.read(),
                is_error: false,
                oninput: move |e: Event<FormData>| {
                    let lat = e.value();
                    param.latitude.set(lat);
                    let _ = check_cord_input(param.latitude, param.latitude_error);
                },
            }
            InputError { error: param.latitude_error.read() }

            label { class: "block font-semibold text-gray-700 mb-1", "Longitude" }
            Input {
                place_holer: Some("e.g. 8.682092".to_string()),
                value: param.longitude.read(),
                is_error: false,
                oninput: move |e: Event<FormData>| {
                    let lon = e.value();
                    param.longitude.set(lon);
                    let _ = check_cord_input(param.longitude, param.longitude_error);
                },
            }
            InputError { error: param.longitude_error.read() }


            label { class: "block font-semibold text-gray-700 mb-1", "Address" }
            Input {
                place_holer: Some("e.g. Main Street 1, 12345 City".to_string()),
                value: param.address.read(),
                is_error: false,
                oninput: move |e: Event<FormData>| {
                    let addr = e.value();
                    param.address.set(addr.clone());
                    let _ = check_addr_input(param.address, param.address_error);
                },
            }
            InputError { error: param.address_error.read() }
        }
    )
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
