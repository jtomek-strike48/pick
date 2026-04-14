//! Standalone test for Knowledge Graph visualization
//! Run with: cargo run --package pentest-desktop --example test_graph

use dioxus::desktop::{Config, LogicalSize, WindowBuilder};
use dioxus::prelude::*;

fn main() {
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            Config::default()
                .with_window(
                    WindowBuilder::new()
                        .with_title("Knowledge Graph Test")
                        .with_inner_size(LogicalSize::new(1200.0, 800.0)),
                )
                .with_custom_head(r#"<style>
                    body { margin: 0; padding: 0; }
                    .test-container { width: 100vw; height: 100vh; display: flex; flex-direction: column; }
                </style>"#.to_string()),
        )
        .launch(TestApp);
}

#[component]
fn TestApp() -> Element {
    rsx! {
        div { class: "test-container",
            h1 { style: "padding: 20px; margin: 0; background: #3B82F6; color: white;",
                "Knowledge Graph Test - Small Engagement (7 nodes)"
            }
            div { style: "flex: 1; padding: 20px;",
                pentest_ui::KnowledgeGraph {
                    engagement_id: "eng-small".to_string(),
                    filters: None
                }
            }
        }
    }
}
