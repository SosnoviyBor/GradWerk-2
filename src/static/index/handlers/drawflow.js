import { editor } from "../main.js";
import { components } from "../components.js"

var mobile_item_selec = ''
var mobile_last_move = null
export function positionMobile(ev) {
    mobile_last_move = ev
}


export function allowDrop(ev) {
    ev.preventDefault()
}


export function drag(ev) {
    if (ev.type === "touchstart") {
        mobile_item_selec = ev.target.closest(".drag-drawflow").getAttribute('data-node')
    } else {
        ev.dataTransfer.setData("node", ev.target.getAttribute('data-node'))
    }
}


export function drop(ev) {
    if (ev.type === "touchend") {
        var parentdrawflow = document
            .elementFromPoint(mobile_last_move.touches[0].clientX, mobile_last_move.touches[0].clientY)
            .closest("#drawflow")
        if (parentdrawflow != null) {
            addNodeToDrawFlow(mobile_item_selec, mobile_last_move.touches[0].clientX, mobile_last_move.touches[0].clientY)
        }
        mobile_item_selec = ''
    } else {
        ev.preventDefault()
        var data = ev.dataTransfer.getData("node")
        addNodeToDrawFlow(data, ev.clientX, ev.clientY)
    }
}


function addNodeToDrawFlow(name, pos_x, pos_y) {
    pos_x = pos_x * (editor.precanvas.clientWidth / (editor.precanvas.clientWidth * editor.zoom)) - (editor.precanvas.getBoundingClientRect().x * (editor.precanvas.clientWidth / (editor.precanvas.clientWidth * editor.zoom)))
    pos_y = pos_y * (editor.precanvas.clientHeight / (editor.precanvas.clientHeight * editor.zoom)) - (editor.precanvas.getBoundingClientRect().y * (editor.precanvas.clientHeight / (editor.precanvas.clientHeight * editor.zoom)))

    const component = components[name]
    switch (name) {
        case "welcome":
            editor.addNode(name, 0, 0, pos_x, pos_y, name, {}, component)
            break

        case "userinput":
            editor.addNode(name, 0, 1, pos_x, pos_y, name, {}, component)
            break

        case "useroutput":
            editor.addNode(name, 1, 0, pos_x, pos_y, name, {}, component)
            break

        case "frontend":
            editor.addNode(name, 1, 1, pos_x, pos_y, name, {}, component)
            break

        case "backend":
            editor.addNode(name, 1, 1, pos_x, pos_y, name, {}, component)
            break

        case "database":
            editor.addNode(name, 1, 1, pos_x, pos_y, name, {}, component)
            break

        case "comment":
            editor.addNode(name, 0, 0, pos_x, pos_y, name, {}, component)
            break

        default:
            console.log("Unexpected component is being added!")
    }
}