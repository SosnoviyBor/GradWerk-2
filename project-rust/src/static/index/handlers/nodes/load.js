import { editor } from "../../main.js"


export function updateNodeLoad(ev) {
    const node = ev.target.closest(".drawflow-node")
    const data = editor.export()["drawflow"]["Home"]["data"][node.id.split("-")[1]]["data"]

    if (!(
        data["deviation"] &&
        data["dist"] &&
        data["mean"] &&
        data["replica"]
    )) {
        return
    }

    fetch("/load", {
        method: "POST",
        body: JSON.stringify({
            deviation: parseFloat(data.deviation),
            dist: data.dist,
            mean: parseFloat(data.mean),
            replica: parseInt(data.replica),
        }),
        headers: { "Content-type": "application/json; charset=UTF-8" }
    })
        .then(response => response.json())
        .then(load => {
            console.log(`Estimated ${node.id} load = ${load}`)
            node.dataset.load = load
            calculateLoadDifference()
        })
}


// Calculates the load of each node relative to the Create node (source node)

// Helper to check if a node is connected (directly or indirectly) to any Create node
function isConnectedToCreate(model, nodeId, createNodeIds, visited = new Set()) {
    if (createNodeIds.has(nodeId)) return true;
    if (visited.has(nodeId)) return false;
    visited.add(nodeId);
    const node = model[nodeId];
    if (!node || !node.inputs) return false;
    for (const input of Object.values(node.inputs)) {
        for (const conn of input.connections) {
            if (isConnectedToCreate(model, conn.node, createNodeIds, visited)) {
                return true;
            }
        }
    }
    return false;
}

export function calculateLoadDifference() {
    const model = editor.export()["drawflow"]["Home"]["data"];

    // Find all Create nodes
    const createNodes = Object.entries(model).filter(
        ([, node]) => node.class === "create"
    );
    if (createNodes.length === 0) return;
    const createNodeIds = new Set(createNodes.map(([id]) => id));

    // TODO REWRITE THIS PIECE OF SHIT

    // For simplicity, use the sum of all Create node loads as the reference load
    let totalCreateLoad = 0;
    for (const [id, node] of createNodes) {
        const el = document.getElementById(`node-${id}`);
        const load = parseFloat(el?.dataset["load"] ?? 0);
        totalCreateLoad += load;
    }
    if (totalCreateLoad === 0) totalCreateLoad = 1; // avoid division by zero

    // For each node, check if it's connected to a Create node
    for (const [current_id, current_node] of Object.entries(model)) {
        const current_node_element = document.getElementById(`node-${current_id}`);
        if (!current_node_element) continue;
        const connected = isConnectedToCreate(model, current_id, createNodeIds);
        // Also disable outline for Dispose and Create nodes
        if (!connected
            || current_node.class === "dispose"
            || current_node === "create"
        ) {
            current_node_element.style.border = "";
            current_node_element.style.boxShadow = "";
            continue;
        }
        const node_load = parseFloat(current_node_element.dataset["load"] ?? 0);
        const load_ratio = node_load / totalCreateLoad;
        updateOutline(current_node_element, load_ratio);
    }
}


function updateOutline(node, load_ratio) {
    if (load_ratio >= 1 || Number.isNaN(load_ratio)) {
        // remove styles
        node.style.border = "";
        node.style.boxShadow = "";
    } else if (load_ratio < 1 && load_ratio >= .8) {
        // color orange
        node.style.border = "1px solid orange";
        node.style.boxShadow = "0 2px 20px 2px orange";
    } else if (load_ratio < .8) {
        // color red
        node.style.border = "1px solid red";
        node.style.boxShadow = "0 2px 20px 2px red";
    }
}
