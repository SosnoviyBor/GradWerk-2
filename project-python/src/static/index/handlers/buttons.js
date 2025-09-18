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
    const model = editor.export()["drawflow"]["Home"]["data"]
    const body = {
        model: model,
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