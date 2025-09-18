import { editor, default_flowchart } from "../main.js";

export function check_unsaved_changes(ev) {
    if (default_flowchart !== editor.export()) {
        ev.preventDefault();
        ev.returnValue = '';
    }
}