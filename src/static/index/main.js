import base_flowchart from "../resources/welcome.json" with { type: 'json' }

// start drawflow
export const default_flowchart = base_flowchart
export const editor = new Drawflow(document.getElementById("drawflow"))

editor.reroute = true
editor.start()
editor.import(default_flowchart)

// zoom out to 80%
editor.zoom_out()
editor.zoom_out()