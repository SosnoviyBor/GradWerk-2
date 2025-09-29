import { editor } from "../main.js";
import * as utils from "../../utils.js";
import * as initializers from "./initializers.js"
import * as node_simstarted from "./nodes/simstarted.js"


const jsonInput = document.getElementById("import-input")
export function importJson() {
    jsonInput.click()
}


export function readImportedJson() {
    const reader = new FileReader()
    reader.onload = parseJson
    reader.readAsText(jsonInput.files[0], "UTF-8")
    jsonInput.value = ""
}


function parseJson(ev) {
    var content
    try {
        content = JSON.parse(ev.target.result)
    } catch (e) {
        alert("Uploaded file is not a valid JSON!")
        return
    }
    editor.import(content)
    initializers.all()
}


export function exportAsJson() {
    const dataStr = "data:text/json;charset=utf-8," + encodeURIComponent(JSON.stringify(editor.export()));
    const downloadAnchorNode = document.createElement('a');
    downloadAnchorNode.setAttribute("href", dataStr);
    downloadAnchorNode.setAttribute("download", "flowchart.json");
    document.body.appendChild(downloadAnchorNode); // required for firefox
    downloadAnchorNode.click();
    downloadAnchorNode.remove();
}


export function clear() {
    if (confirm("Are you sure you want to clear your flowchart?")) {
        editor.clearModuleSelected()
    }
}


export function requestSimulation() {
    const simtime = Number(document.getElementById("simtime").value)
    const log_max_size = Number(document.getElementById("log-max-size").value)

    // check params
    if (!(
        Number.isInteger(simtime) &&
        simtime > 0 &&
        Number.isInteger(log_max_size) &&
        log_max_size > 0
    )) {
        console.log(`
            Wrong input!
            simtime = ${simtime}
            logmaxsize = ${log_max_size}
        `)
        return
    }

    // show overlay
    document.getElementById("sim-started-overlay").hidden = false
    node_simstarted.showOverlay()

    // start simulation
    const body = {
        model: prepareModel(),
        simtime: simtime,
        log_max_size: log_max_size
    }
    console.log(body)

    fetch("/simulate", {
        method: "POST",
        body: JSON.stringify(body),
        headers: { "Content-type": "application/json; charset=UTF-8" }
    })
        .then(response => response.json())
        .then(json => {
            utils.postToNewTab(json, "/results")
            node_simstarted.hideOverlay()
        })
}

function prepareModel() {
    const model = editor.export()["drawflow"]["Home"]["data"]
    for (const [id, e] of Object.entries(model)) {
        // clear redundant fields
        delete e.html
        delete e.id
        delete e.name
        delete e.pos_x
        delete e.pos_y
        delete e.typenode

        // edit element data
        switch (e.class) {
            case "create":
            case "process":
                // cast values into the correct types
                e.data.deviation = Number(e.data.deviation)
                e.data.mean = Number(e.data.mean)
                e.data.queuesize = Number(e.data.queuesize)
                e.data.replica = Number(e.data.replica)
                break
            case "dispose":
                // populate with dummy values
                e.data.deviation = 0
                e.data.dist = "exponential"
                e.data.mean = 0
                e.data.order = "balanced"
                e.data.queuesize = 0
                e.data.replica = 0
                break
            default:
                // delete everything else
                delete model[id]
                break
        }
        // resturcture IO
        e.inputs = Object.values(e.inputs)
            .flatMap(input => input.connections.map(c => Number(c.node)));
        e.outputs = Object.values(e.outputs)
            .flatMap(output => output.connections.map(c => Number(c.node)));
    }

    return model
}