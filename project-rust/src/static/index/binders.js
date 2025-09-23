import { editor } from "./main.js";
import * as drawflow_handlers from "./handlers/drawflow.js"
import * as button_handlers from "./handlers/buttons.js"
import * as window_handlers from "./handlers/window.js"
import * as node_load from "./handlers/nodes/load.js"
import * as node_simstarted from "./handlers/nodes/simstarted.js"
import * as initializers from "./handlers/initializers.js"

/* element binders */

Array.from(document.getElementsByClassName('drag-drawflow')).forEach(element => {
    element.addEventListener('touchend', drawflow_handlers.drop, false);
    element.addEventListener('touchmove', drawflow_handlers.positionMobile, false);
    element.addEventListener('touchstart', drawflow_handlers.drag, false);
    element.addEventListener('dragstart', ev => drawflow_handlers.drag(ev))
})

document.getElementById("drawflow").addEventListener('dragover', ev => drawflow_handlers.allowDrop(ev))
document.getElementById("drawflow").addEventListener('drop', ev => drawflow_handlers.drop(ev))

document.getElementById("btn-export").addEventListener("click", button_handlers.exportAsJson)
document.getElementById("btn-import").addEventListener("click", button_handlers.importJson)
document.getElementById("import-input").addEventListener("change", button_handlers.readImportedJson)
document.getElementById("btn-clear").addEventListener("click", button_handlers.clear)

document.getElementById("start").addEventListener("click", button_handlers.requestSimulation)
document.getElementById("sim-started-overlay").addEventListener("click", node_simstarted.hideOverlay)
document.getElementById("header").addEventListener("click", node_simstarted.hideOverlay)



/* window binders */

window.addEventListener('beforeunload', ev => window_handlers.check_unsaved_changes(ev));



/* editor binders */

editor.on('nodeCreated',        id => console.log("Node created " + id))
editor.on('nodeRemoved',        id => console.log("Node removed " + id))
editor.on('nodeSelected',       id => console.log("Node selected " + id))
editor.on('nodeDataChanged',    id => console.log("Updated data in node  "+ id))
editor.on('moduleCreated',      name => console.log("Module Created " + name))
editor.on('moduleChanged',      name => console.log("Module Changed " + name))
editor.on('nodeMoved',          id => console.log("Node moved " + id))
editor.on('addReroute',         id => console.log("Reroute added " + id))
editor.on('removeReroute',      id => console.log("Reroute removed " + id))
editor.on('connectionCreated',  connection => {
    console.log('Connection created')
    console.log(connection)
    node_load.calculateLoadDifference()
})
editor.on('connectionRemoved', connection => {
    console.log('Connection removed')
    console.log(connection)
    node_load.calculateLoadDifference()
})
editor.on("nodeCreated", id => {
    initializers.nodeData(id)
    initializers.dblclickboxListeners(id)
})

// init all loaded nodes' stuff
initializers.all()