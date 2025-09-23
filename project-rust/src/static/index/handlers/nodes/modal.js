import { editor } from "../../main.js";


var transform = '';
const default_zoom_value = editor.zoom_value
const default_zoom_min = editor.zoom_min
const default_zoom_max = editor.zoom_max
export function showModal(ev) {
    // to prevent doublickicks registering on any inner tags
    // not that they should be present in nodes anyway
    // but, well, it looks neat
    if (!ev.target.classList.contains("box")) { return }

    editor.zoom_value = 0
    editor.zoom_min = editor.zoom
    editor.zoom_max = editor.zoom
    ev.target.closest(".drawflow-node").style.zIndex = "9999";
    ev.target.children[0].style.display = "block";
    transform = editor.precanvas.style.transform;
    editor.precanvas.style.transform = '';
    editor.precanvas.style.left = editor.canvas_x +'px';
    editor.precanvas.style.top = editor.canvas_y +'px';
}


export function closeModal(ev) {
    ev.target.closest(".drawflow-node").style.zIndex = "2";
    ev.target.parentElement.parentElement.style.display  = "none";
    editor.precanvas.style.transform = transform;
    editor.precanvas.style.left = '0px';
    editor.precanvas.style.top = '0px';
    editor.zoom_value = default_zoom_value
    editor.zoom_min = default_zoom_min
    editor.zoom_max = default_zoom_max
}