use icondata::Icon;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_icons::*; // Import the leptos-icons library
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

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
fn ServiceStatus(status: bool) -> impl IntoView {
    let status_color = move || {
        if status {
            "green" // Service is active
        } else {
            "red" // Service is inactive
        }
    };
    view! {
        <div class="service-status">
            <div
                class="status-indicator"
                style=format!("background-color: {}", status_color())
            ></div>
            <span class="status-text">
                {move || format!("SNEK HQ {}", if status { "Online" } else { "Offline" })}
            </span>
        </div>
    }
}

#[component]
fn Header() -> impl IntoView {
    let (service_status, set_service_status) = signal(true); // true = online, false = offline

    view! {
        <header class="header">
            <div class="header-left">
                <ServiceStatus status=service_status.get() />
            </div>
            <div class="header-right">
                <img
                    src="./icons/snek_icon_48.png" // Replace with your image URL
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
    let active_tab = RwSignal::new(0); // Use create_rw_signal to create an RwSignal

    //view! { <HomeContent />};

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
