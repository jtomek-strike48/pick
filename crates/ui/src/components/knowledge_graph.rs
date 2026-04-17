//! Knowledge Graph Visualization Component
//!
//! Displays evidence chains from StrikeKit as an interactive graph using Cytoscape.js.

use dioxus::prelude::*;
use pentest_core::evidence::{
    filters::apply_filters,
    mock::MockEvidenceClient,
    transformer::transform_to_cytoscape,
    types::{EvidenceFilters, KnowledgeGraph},
};

/// Props for the KnowledgeGraph component
#[derive(Props, Clone, PartialEq)]
pub struct KnowledgeGraphProps {
    /// Engagement ID to display
    pub engagement_id: String,

    /// Optional filters to apply
    #[props(default)]
    pub filters: Option<EvidenceFilters>,
}

/// Knowledge Graph visualization component
///
/// Renders an interactive graph of evidence chains using Cytoscape.js.
/// Supports filtering, node selection, and detail viewing.
#[component]
pub fn KnowledgeGraph(props: KnowledgeGraphProps) -> Element {
    // State: raw graph data from API
    let mut graph_data = use_signal(|| None::<KnowledgeGraph>);

    // State: selected node ID
    let mut selected_node = use_signal(|| None::<String>);

    // State: loading status
    let mut is_loading = use_signal(|| true);

    // State: error message
    let mut error_message = use_signal(|| None::<String>);

    // Fetch graph data on mount or when engagement_id changes
    let engagement_id = props.engagement_id.clone();
    let filters = props.filters.clone();

    use_effect(move || {
        let engagement_id = engagement_id.clone();
        let filters = filters.clone();

        spawn(async move {
            is_loading.set(true);
            error_message.set(None);

            // Fetch from mock API (will replace with real API later)
            let client = MockEvidenceClient::new();

            match client.fetch_evidence_chain(&engagement_id, filters.clone()).await {
                Ok(mut graph) => {
                    // Apply filters if provided
                    if let Some(ref filter_opts) = filters {
                        graph = apply_filters(&graph, &engagement_id, filter_opts);
                    }

                    graph_data.set(Some(graph));
                    is_loading.set(false);
                }
                Err(e) => {
                    error_message.set(Some(format!("Failed to load evidence chain: {}", e)));
                    is_loading.set(false);
                }
            }
        });
    });

    // Transform graph data to Cytoscape format
    let cytoscape_json = use_memo(move || {
        graph_data.read().as_ref().map(|graph| {
            let cyto_graph = transform_to_cytoscape(graph);
            serde_json::to_string(&cyto_graph).unwrap_or_else(|_| "{}".to_string())
        })
    });

    rsx! {
        style { {include_str!("css/knowledge_graph.css")} }

        div { class: "knowledge-graph-container",
            // Loading state
            if *is_loading.read() {
                div { class: "loading-overlay",
                    div { class: "loading-spinner" }
                    p { "Loading evidence chain..." }
                }
            }

            // Error state
            if let Some(ref error) = *error_message.read() {
                div { class: "error-message",
                    h3 { "Error" }
                    p { "{error}" }
                }
            }

            // Graph container
            if !*is_loading.read() && error_message.read().is_none() {
                div { class: "graph-wrapper",
                    // Zoom controls
                    div { class: "graph-controls",
                        button {
                            class: "zoom-btn zoom-in",
                            title: "Zoom In",
                            onclick: move |_| {
                                let script = r#"
                                    if (window.cytoscapeInstance) {
                                        window.cytoscapeInstance.zoom({
                                            level: window.cytoscapeInstance.zoom() * 1.2,
                                            renderedPosition: {
                                                x: window.cytoscapeInstance.width() / 2,
                                                y: window.cytoscapeInstance.height() / 2
                                            }
                                        });
                                    }
                                "#;
                                spawn(async move {
                                    let _ = document::eval(script).await;
                                });
                            },
                            "+"
                        }
                        button {
                            class: "zoom-btn zoom-out",
                            title: "Zoom Out",
                            onclick: move |_| {
                                let script = r#"
                                    if (window.cytoscapeInstance) {
                                        window.cytoscapeInstance.zoom({
                                            level: window.cytoscapeInstance.zoom() / 1.2,
                                            renderedPosition: {
                                                x: window.cytoscapeInstance.width() / 2,
                                                y: window.cytoscapeInstance.height() / 2
                                            }
                                        });
                                    }
                                "#;
                                spawn(async move {
                                    let _ = document::eval(script).await;
                                });
                            },
                            "−"
                        }
                        button {
                            class: "zoom-btn zoom-fit",
                            title: "Fit to Screen",
                            onclick: move |_| {
                                let script = r#"
                                    if (window.cytoscapeInstance) {
                                        window.cytoscapeInstance.fit(null, 50);
                                    }
                                "#;
                                spawn(async move {
                                    let _ = document::eval(script).await;
                                });
                            },
                            "⊡"
                        }
                    }

                    // Cytoscape.js will be injected here
                    div {
                        id: "cy",
                        class: "cytoscape-container",

                    }

                    // Load Cytoscape.js and dagre layout from CDN
                    script {
                        src: "https://unpkg.com/cytoscape@3.28.1/dist/cytoscape.min.js",
                        r#type: "text/javascript"
                    }
                    script {
                        src: "https://unpkg.com/dagre@0.8.5/dist/dagre.min.js",
                        r#type: "text/javascript"
                    }
                    script {
                        src: "https://unpkg.com/cytoscape-dagre@2.5.0/cytoscape-dagre.js",
                        r#type: "text/javascript"
                    }

                    // Initialize Cytoscape.js
                    if let Some(ref json) = *cytoscape_json.read() {
                        script {
                            dangerous_inner_html: "{generate_init_script(json)}"
                        }
                    }
                }

                // Node detail panel (if node selected)
                if let Some(ref node_id) = *selected_node.read() {
                    NodeDetailPanel {
                        graph: graph_data.read().clone(),
                        node_id: node_id.clone(),
                        on_close: move |_| selected_node.set(None)
                    }
                }

                // Graph stats
                if let Some(ref graph) = *graph_data.read() {
                    div { class: "graph-stats",
                        span { class: "stat-item",
                            strong { "Nodes: " }
                            "{graph.node_count()}"
                        }
                        span { class: "stat-item",
                            strong { "Edges: " }
                            "{graph.edge_count()}"
                        }
                    }
                }
            }
        }
    }
}

/// Generate Cytoscape.js initialization script
fn generate_init_script(json_data: &str) -> String {
    format!(
        r#"
        (function() {{
            console.log('[CYTO] Step 1: Initializing Cytoscape.js...');

            console.log('[CYTO] Step 2: Checking cytoscape library...');
            if (typeof cytoscape === 'undefined') {{
                console.error('[CYTO] ERROR: Cytoscape.js not loaded - waiting and retrying...');
                setTimeout(arguments.callee, 100);
                return;
            }}
            console.log('[CYTO] Step 3: Cytoscape.js is loaded');

            console.log('[CYTO] Step 4: Checking dagre library...');
            if (typeof dagre === 'undefined') {{
                console.error('[CYTO] ERROR: Dagre not loaded - waiting and retrying...');
                setTimeout(arguments.callee, 100);
                return;
            }}
            console.log('[CYTO] Step 5: Dagre is loaded');

            console.log('[CYTO] Step 6: Checking for container #cy...');
            const container = document.getElementById('cy');
            if (!container) {{
                console.error('[CYTO] ERROR: Container #cy not found - waiting and retrying...');
                setTimeout(arguments.callee, 100);
                return;
            }}
            console.log('[CYTO] Step 7: Container found:', container);

            console.log('[CYTO] Step 8: Parsing graph data...');
            const data = {json_data};
            console.log('[CYTO] Step 9: Graph data parsed:', data);
            console.log('[CYTO] Elements count:', data.elements ? data.elements.length : 'NO ELEMENTS');

            console.log('[CYTO] Step 10: Registering dagre layout...');
            if (typeof CytoscapeDagre !== 'undefined') {{
                cytoscape.use(CytoscapeDagre);
                console.log('[CYTO] Step 11: Dagre layout registered');
            }} else {{
                console.warn('[CYTO] WARNING: CytoscapeDagre not found, trying without it');
            }}

            console.log('[CYTO] Step 12: Creating Cytoscape instance...');
            try {{
                const cy = cytoscape({{
                container: container,
                elements: data.elements,
                style: [
                    {{
                        selector: 'node',
                        style: {{
                            'label': 'data(label)',
                            'background-color': 'data(backgroundColor)',
                            'border-color': 'data(borderColor)',
                            'border-width': 3,
                            'shape': 'data(shape)',
                            'width': 120,
                            'height': 120,
                            'text-valign': 'center',
                            'text-halign': 'center',
                            'font-size': '14px',
                            'font-weight': 'bold',
                            'color': '#fff',
                            'text-outline-color': '#000',
                            'text-outline-width': 2,
                            'text-wrap': 'wrap',
                            'text-max-width': '110px'
                        }}
                    }},
                    {{
                        selector: 'edge',
                        style: {{
                            'width': 3,
                            'line-color': 'data(lineColor)',
                            'target-arrow-color': 'data(lineColor)',
                            'target-arrow-shape': 'triangle',
                            'curve-style': 'bezier',
                            'arrow-scale': 2
                        }}
                    }},
                    {{
                        selector: ':selected',
                        style: {{
                            'border-width': 5,
                            'border-color': '#FFD700'
                        }}
                    }}
                ],
                layout: {{
                    name: 'dagre',
                    rankDir: 'TB',
                    nodeSep: 80,
                    rankSep: 150,
                    animate: true,
                    animationDuration: 500
                }},
                minZoom: 0.3,
                maxZoom: 3,
                wheelSensitivity: 0.2
            }});

                console.log('[CYTO] Step 13: Cytoscape instance created successfully!');
                console.log('[CYTO] Step 14: Node count:', cy.nodes().length);
                console.log('[CYTO] Step 15: Edge count:', cy.edges().length);

                // Handle node click
                cy.on('tap', 'node', function(evt) {{
                    const node = evt.target;
                    const nodeId = node.id();
                    console.log('[CYTO] Node clicked:', nodeId);
                }});

                // Store cy instance globally for access
                window.cytoscapeInstance = cy;
                console.log('[CYTO] Step 16: INITIALIZATION COMPLETE!');
            }} catch (error) {{
                console.error('[CYTO] ERROR during initialization:', error);
                console.error('[CYTO] Error stack:', error.stack);
            }}
        }})();
        "#,
        json_data = json_data
    )
}

/// Node detail panel component (placeholder for now)
#[component]
fn NodeDetailPanel(
    graph: Option<KnowledgeGraph>,
    node_id: String,
    on_close: EventHandler<()>,
) -> Element {
    let node = graph.as_ref().and_then(|g| g.find_node(&node_id));

    rsx! {
        div { class: "node-detail-panel",
            div { class: "panel-header",
                h3 { "Node Details" }
                button {
                    class: "close-button",
                    onclick: move |_| on_close.call(()),
                    "×"
                }
            }

            if let Some(node) = node {
                div { class: "panel-content",
                    div { class: "detail-row",
                        strong { "ID: " }
                        span { "{node.id}" }
                    }
                    div { class: "detail-row",
                        strong { "Type: " }
                        span { "{node.node_type.label()}" }
                    }
                    div { class: "detail-row",
                        strong { "Title: " }
                        span { "{node.title}" }
                    }
                    div { class: "detail-row",
                        strong { "Description: " }
                        p { "{node.description}" }
                    }
                    div { class: "detail-row",
                        strong { "Confidence: " }
                        span { "{node.confidence:.2}" }
                    }
                    div { class: "detail-row",
                        strong { "Created: " }
                        span { "{node.timestamp}" }
                    }
                    div { class: "detail-row",
                        strong { "Created By: " }
                        span { "{node.created_by}" }
                    }
                    if let Some(ref target) = node.target {
                        div { class: "detail-row",
                            strong { "Target: " }
                            span { "{target}" }
                        }
                    }
                    div { class: "detail-row",
                        strong { "Metadata: " }
                        pre { {serde_json::to_string_pretty(&node.metadata).unwrap_or_default()} }
                    }
                }
            } else {
                div { class: "panel-content",
                    p { "Node not found" }
                }
            }
        }
    }
}
