use icondata::Icon;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_icons::*; // Import the leptos-icons library
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::console::log;

#[derive(Serialize, Deserialize, Clone)]
struct StatusResponse {
    status: bool, // true = online, false = offline
}

// Global state for service status
#[derive(Copy, Clone)]
struct ServiceStatus {
    status: RwSignal<bool>,
}

impl ServiceStatus {
    fn new() -> Self {
        Self {
            status: RwSignal::new(false), // Default to offline
        }
    }
}

// Global state for UID
#[derive(Copy, Clone)]
struct UidState {
    uid: RwSignal<Option<String>>,
}

impl UidState {
    fn new() -> Self {
        Self {
            uid: RwSignal::new(None), // Default to None
        }
    }
}

// Function to check the service status
async fn check_status() -> Result<bool, String> {
    let response = reqwest::get("http://localhost:8080/status")
        .await
        .map_err(|_| "Failed to fetch status".to_string())?;

    let status_response: StatusResponse = response
        .json()
        .await
        .map_err(|_| "Failed to parse status response".to_string())?;

    Ok(status_response.status)
}

// Background task to periodically check the service status
async fn status_checker(service_status: ServiceStatus) {
    loop {
        match check_status().await {
            Ok(status) => {
                service_status.status.set(status);
            }
            Err(err) => {
                log!("{}", err);
                service_status.status.set(false); // Set status to offline on error
            }
        }

        // Wait for 5 seconds before checking again
        gloo_timers::future::sleep(std::time::Duration::from_secs(5)).await;
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct UidResponse {
    uid: String,
}

// Function to fetch UID from the API
async fn fetch_uid() -> Result<String, String> {
    let response = reqwest::get("http://localhost:8080/generate_uid")
        .await
        .map_err(|_| "Failed to fetch UID".to_string())?;

    let uid_response: UidResponse = response
        .json()
        .await
        .map_err(|_| "Failed to parse UID response".to_string())?;

    Ok(uid_response.uid)
}

// Function to get or generate UID
async fn get_or_generate_uid(uid_state: UidState, service_status: ServiceStatus) {
    // Check localStorage for SNEK_HQ_UID
    if let Some(uid) = localStorage_get("SNEK_HQ_UID") {
        uid_state.uid.set(Some(uid));
        return;
    }

    // If not found, fetch UID from the API
    match fetch_uid().await {
        Ok(uid) => {
            // Store the UID in localStorage
            localStorage_set("SNEK_HQ_UID", &uid);
            uid_state.uid.set(Some(uid));
        }
        Err(err) => {
            log!("{}", err);
            // Start a retry mechanism if the service is online
            if service_status.status.get() {
                spawn_local(uid_retry_mechanism(uid_state, service_status));
            }
        }
    }
}

// Background task to retry fetching UID
async fn uid_retry_mechanism(uid_state: UidState, service_status: ServiceStatus) {
    loop {
        // Wait for 5 seconds before retrying
        gloo_timers::future::sleep(std::time::Duration::from_secs(5)).await;

        // Only retry if the service is online
        if !service_status.status.get() {
            break;
        }

        match fetch_uid().await {
            Ok(uid) => {
                // Store the UID in localStorage
                localStorage_set("SNEK_HQ_UID", &uid);
                uid_state.uid.set(Some(uid));
                break; // Exit the retry loop on success
            }
            Err(err) => {
                log!("{}", err);
            }
        }
    }
}

// Helper function to get a value from localStorage
fn localStorage_get(key: &str) -> Option<String> {
    let window = web_sys::window()?;
    let localStorage = window.local_storage().ok()??;
    localStorage.get_item(key).ok()?
}

// Helper function to set a value in localStorage
fn localStorage_set(key: &str, value: &str) {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(local_storage)) = window.local_storage() {
            let _ = local_storage.set_item(key, value);
        }
    }
}

#[component]
fn Tab(
    icon: icondata::Icon, // Correct usage of the Icon type
    hover_text: String,
    is_active: Box<dyn Fn() -> bool + Send + Sync>, // Use Box<dyn Fn() -> bool + Send + Sync>
    on_click: impl Fn() + 'static, // Use a generic function type instead of Callback
) -> impl IntoView {
    let tab_class = move || {
        if is_active() {
            "tab active"
        } else {
            "tab"
        }
    };
    view! {
        <div class=tab_class on:click=move |_| on_click()>
            <Icon icon=icon />
            <span class="hover-text">{hover_text}</span>
        </div>
    }
}

#[component]
fn Tabs(active_tab: RwSignal<usize>) -> impl IntoView {
    let tabs = vec![
        (Icon::from(icondata::SiHomeassistant), "Home".to_string()), // Create Icon instance
        (
            Icon::from(icondata::AiAlertOutlined),
            "RaidCenter".to_string(),
        ), // Create Icon instance
        (Icon::from(icondata::FiSettings), "Settings".to_string()),  // Create Icon instance
    ];

    // Use Effect::new to log the active tab whenever it changes
    Effect::new(move |_| {
        log!("Active tab changed to: {}", active_tab.get());
    });

    view! {
        <div class="tabs-container">
            {tabs.into_iter().enumerate().map(|(index, (icon, text))| {
                let is_active = Box::new(move || active_tab.get() == index) as Box<dyn Fn() -> bool + Send + Sync>; // Wrap in Box
                let on_click = move || active_tab.set(index); // Use a closure instead of Callback

                view! {
                    <Tab
                        icon=icon
                        hover_text=text.clone()
                        is_active=is_active
                        on_click=on_click
                    />
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

// Content Components
#[component]
fn HomeContent() -> impl IntoView {
    view! {
        <div class="content">
            <h1>"Welcome to the Home Tab"</h1>
            <p>"This is the home content."</p>
        </div>
    }
}

#[component]
fn SettingsContent() -> impl IntoView {
    view! {
        <div class="content">
            <h1>"Settings Tab"</h1>
            <p>"Adjust your settings here."</p>
        </div>
    }
}

#[component]
fn RaidCenterContent() -> impl IntoView {
    view! {
        <div class="content">
            <h1>"RaidCenter Tab"</h1>
            <p>"View and do Raiding Tasks."</p>
        </div>
    }
}

#[component]
fn ServiceStatus() -> impl IntoView {
    let service_status = use_context::<ServiceStatus>().expect("ServiceStatus context not found");

    let status_color = move || {
        if service_status.status.get() {
            "green" // Online
        } else {
            "red" // Offline
        }
    };

    view! {
        <div class="service-status">
            <div
                class="status-indicator"
                style=format!("background-color: {}", status_color())
            ></div>
            <span class="status-text">
                {move || format!("SNEK HQ {}", if service_status.status.get() { "Online" } else { "Offline" })}
            </span>
        </div>
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <header class="header">
            <div class="header-left">
                <ServiceStatus />
            </div>
            <div class="header-right">
                <img
                    src="./icons/snek_icon_48.png"
                    alt="Logo"
                    class="header-image"
                />
            </div>
        </header>
    }
}

#[component]
fn Footer() -> impl IntoView {
    let app_version = "1.0.0"; // Replace with your actual app version

    view! {
        <footer class="footer">
            <p class="footer-text">"Version: " {app_version}</p>
        </footer>
    }
}

// Main App Component
#[component]
fn App() -> impl IntoView {
    let service_status = ServiceStatus::new();
    let uid_state = UidState::new();

    // Start the background status checker
    spawn_local(status_checker(service_status));

    // Fetch or generate UID when the app launches
    spawn_local(get_or_generate_uid(uid_state, service_status));

    // Provide global states to the app
    provide_context(service_status);
    provide_context(uid_state);
    let active_tab = RwSignal::new(0); // Use create_rw_signal to create an RwSignal

    view! {
        <Header />
        <div class="app-container">
            <Tabs active_tab=active_tab />
            <div class="content-container">
                <Show when=move || active_tab.get() == 0>
                    <HomeContent />
                </Show>
                <Show when=move || active_tab.get() == 1>
                    <RaidCenterContent />
                </Show>
                <Show when=move || active_tab.get() == 2>
                    <SettingsContent />
                </Show>
            </div>
            <Footer />
        </div>
    }
}

#[wasm_bindgen]
pub fn run() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
