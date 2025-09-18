import { editor } from "../main.js"
import * as node_load from "./nodes/load.js"
import * as node_modal from "./nodes/modal.js"
import * as node_general from "./nodes/general.js"

export function nodeData(id) {
    if (!document.getElementById(`node-${id}`).querySelector(".dbclickbox")) { return }

    const updateable_tags = ["input", "select", "textarea"]

    const content = document.getElementById(`node-${id}`).querySelector(".modal-content")
    const event = new Event("input", { bubbles: true })

    Array.from(content.children).forEach((child) => {
        if (updateable_tags.includes(child.tagName.toLowerCase())) {
            child.dispatchEvent(event)
        }
    })

    node_load.updateNodeLoad({ target: content })
}


export function dblclickboxListeners(id) {
    const box = document.getElementById(`node-${id}`).getElementsByClassName("box")[0]

    if (!box.classList.contains("dbclickbox")) { return }

    box.addEventListener("dblclick", ev => node_modal.showModal(ev))
    box.querySelector(".modal-close").addEventListener("click", ev => {
        node_modal.closeModal(ev)
    node_load.updateNodeLoad(ev)
    })
    box.querySelector(".df-name").addEventListener("change", ev => node_general.updateName(ev))
}


export function all() {
    Array.from(Object.keys(editor.export()["drawflow"]["Home"]["data"]),
        id => {
            dblclickboxListeners(id)
            nodeData(id)
            const dfname = document.getElementById(`node-${id}`)
                                   .getElementsByClassName("box")[0]
                                   .querySelector(".df-name")
            if (dfname) {
                dfname.dispatchEvent(new Event("change"))
            }
        }
    )
}