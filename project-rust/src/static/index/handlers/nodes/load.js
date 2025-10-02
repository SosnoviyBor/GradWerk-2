import { ElementOrder } from "../../../consts.js"
import { editor } from "../../main.js"


export async function updateNodeCapacity(ev) {
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

    await fetch("/capacity", {
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
        .then(capacity => {
            console.log(`Estimated ${node.id} capacity = ${capacity}`)
            node.dataset.capacity = capacity
        })
}

export function updateAllLoads() {
    const model = editor.export()["drawflow"]["Home"]["data"];

    // Find all Create nodes
    const createNodes = Object.values(model).filter(
        node => node.class === "create"
    );
    if (createNodes.length === 0) return;

    // init all load values
    Object.keys(model).forEach(id => {
        const node = document.getElementById(`node-${id}`)
        if (node.classList.contains("create")) {
            node.dataset.load = node.dataset.capacity
        } else {
            node.dataset.load = 0
        }
    });
    // start with create nodes (recursive)
    evaluateChildrenLoad(createNodes);
    // update outline
    const process_nodes = document.getElementsByClassName("drawflow-node process")
    Object.values(process_nodes).forEach(node => {
        const load = Number(node.dataset.load)
        const capacity = Number(node.dataset.capacity)
        updateOutline(node, load / capacity)
    })
}

function evaluateChildrenLoad(nodes) {
    Object.values(nodes).forEach(parent_node => {
        // get children list
        const children_ids = Object.values(parent_node.outputs).flatMap(output =>
            Object.values(output.connections)
                .flatMap(conn => (conn.node ? [conn.node] : []))
        );
        if (children_ids.length === 0) return;
        // distribute load between the children
        const parent_el = document.getElementById(`node-${parent_node.id}`)
        switch (parent_node.data.order) {
            // the load depends on child's capacity
            case ElementOrder.balanced:
                const init_capacity = 0
                const children_total_capacity = Object.values(children_ids).reduce(
                    (acc, id) => acc + Number(document.getElementById(`node-${id}`).dataset.capacity),
                    init_capacity
                )
                Object.values(children_ids).forEach(id => {
                    const child_el = document.getElementById(`node-${id}`)
                    // fucking hell js has long nameplates
                    const parent_load = Number(parent_el.dataset.load);
                    const child_capacity = Number(child_el.dataset.capacity);
                    const child_share = child_capacity / children_total_capacity;
                    const curr_load = Number(child_el.dataset.load)
                    child_el.dataset.load = curr_load + parent_load * child_share
                })
                break
            // the load is spread evenly
            case ElementOrder.random:
            case ElementOrder.round_robin:
                Object.values(children_ids).forEach(id => {
                    const child_el = document.getElementById(`node-${id}`)
                    const parent_load = Number(parent_el.dataset.load);
                    const child_count = children_ids.length
                    const curr_load = Number(child_el.dataset.load)
                    child_el.dataset.load = curr_load + parent_load / child_count
                })
                break
        }
        // recursion!
        evaluateChildrenLoad(
            Object.values(children_ids).flatMap(
                id => editor.export().drawflow.Home.data[id]
            )
        );
    })
}

function updateOutline(node, load_ratio) {
    if (load_ratio >= 1) {
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
