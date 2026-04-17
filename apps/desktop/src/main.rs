//! Pentest Connector Desktop Application

use dioxus::desktop::{Config, LogicalSize, WindowBuilder};
use dioxus::prelude::*;

use pentest_core::config::ShellMode;
use pentest_core::settings::load_settings;
use pentest_ui::{connector_app, mobile_css, utils_css, ConnectorAppConfig};

mod graph_window;
mod graph_bridge;

use graph_window::GraphMessage;
pub use graph_bridge::{send_clear, send_loading, send_sample_graph};

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Mutex;

/// Global channel for sending graph updates to the egui window
///
/// # Thread Safety
/// - Initialized once on startup (main thread)
/// - Sender cloned and stored here for access from Dioxus components
/// - Protected by Mutex for safe concurrent access
/// - Never cleared after initialization (Some(...) for application lifetime)
static GRAPH_CHANNEL: Mutex<Option<mpsc::SyncSender<GraphMessage>>> = Mutex::new(None);

/// Shutdown flag to prevent error logging during clean shutdown
static SHUTDOWN: AtomicBool = AtomicBool::new(false);

/// Send a message to the graph window
pub fn send_to_graph_window(msg: GraphMessage) {
    // Don't log errors during shutdown
    if SHUTDOWN.load(Ordering::Relaxed) {
        return;
    }

    match GRAPH_CHANNEL.lock() {
        Ok(guard) => {
            if let Some(ref sender) = *guard {
                // Use try_send for bounded channel to handle backpressure
                match sender.try_send(msg) {
                    Ok(()) => {}
                    Err(mpsc::TrySendError::Full(_)) => {
                        tracing::warn!("Graph window channel full, dropping message");
                    }
                    Err(mpsc::TrySendError::Disconnected(_)) => {
                        if !SHUTDOWN.load(Ordering::Relaxed) {
                            tracing::debug!("Graph window disconnected");
                        }
                    }
                }
            }
        }
        Err(e) => {
            tracing::error!("Graph channel mutex poisoned: {}", e);
        }
    }
}

const DESKTOP_CONFIG: ConnectorAppConfig = ConnectorAppConfig {
    platform_name: "Desktop",
    container_class: "mobile-app",
    shell_route_mode: ShellMode::Native,
    default_proot: false,
    start_liveview_server: true,
    inject_css: false,
    extra_init_messages: &[],
    create_tools: pentest_tools::create_tool_registry,
    set_sandbox: Some(pentest_platform::set_use_sandbox),
};

fn main() {
    // Initialize logging: console + file
    let log_path = pentest_core::logging::init_logging_with_file("debug");

    tracing::info!("Log file: {}", log_path.display());
    tracing::info!("Starting Pentest Connector Desktop");

    // Create bounded channel for graph updates (max 100 pending messages)
    // This prevents unbounded memory growth if updates come faster than egui can process
    let (tx, rx) = mpsc::sync_channel(100);

    // Store sender in global static for access from Dioxus components
    {
        let mut channel = GRAPH_CHANNEL.lock().unwrap();
        *channel = Some(tx);
    }

    // Launch Dioxus window in separate thread (egui must be on main thread)
    // On Linux, we use the platform-specific with_any_thread() to allow EventLoop creation off main thread
    let dioxus_thread = std::thread::spawn(move || {
        // Load theme from settings
        let settings = load_settings();
        let css = pentest_ui::theme::generate_theme_css(
            settings.theme,
            settings.border_radius,
            settings.density,
        );
        let mcss = mobile_css();
        let ucss = utils_css();

        #[cfg(target_os = "linux")]
        {
            use dioxus::desktop::tao::event_loop::EventLoopBuilder;
            use dioxus::desktop::tao::platform::unix::EventLoopBuilderExtUnix;

            // Create EventLoop with any_thread() for Linux compatibility
            let event_loop = EventLoopBuilder::with_user_event()
                .with_any_thread(true)
                .build();

            let config = Config::default()
                .with_event_loop(event_loop)
                .with_window(
                    WindowBuilder::new()
                        .with_title("Pentest Connector")
                        .with_inner_size(LogicalSize::new(480.0, 800.0))
                        .with_min_inner_size(LogicalSize::new(360.0, 600.0))
                )
                .with_custom_head(format!(r#"<style>{css}{mcss}{ucss}</style>"#));

            dioxus::LaunchBuilder::desktop()
                .with_cfg(config)
                .launch(DesktopApp);
        }

        #[cfg(not(target_os = "linux"))]
        {
            dioxus::LaunchBuilder::desktop()
                .with_cfg(
                    Config::default()
                        .with_window(
                            WindowBuilder::new()
                                .with_title("Pentest Connector")
                                .with_inner_size(LogicalSize::new(480.0, 800.0))
                                .with_min_inner_size(LogicalSize::new(360.0, 600.0))
                        )
                        .with_custom_head(format!(r#"<style>{css}{mcss}{ucss}</style>"#)),
                )
                .launch(DesktopApp);
        }
    });

    tracing::info!("Dioxus window launched in separate thread");

    // Launch graph window on main thread (required for Linux)
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("Knowledge Graph - Pick"),
        ..Default::default()
    };

    let window = graph_window::GraphWindow::new(rx);

    tracing::info!("Launching Knowledge Graph window on main thread");
    if let Err(e) = eframe::run_native(
        "Knowledge Graph - Pick",
        options,
        Box::new(|_cc| Ok(Box::new(window))),
    ) {
        tracing::error!("Failed to run graph window: {}", e);
    }

    // Signal shutdown to prevent error logging when Dioxus tries to send messages
    SHUTDOWN.store(true, Ordering::Relaxed);

    // Wait for Dioxus thread to finish
    let _ = dioxus_thread.join();
}

#[component]
fn DesktopApp() -> Element {
    connector_app(DESKTOP_CONFIG)
}
