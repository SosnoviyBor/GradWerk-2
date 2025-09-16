import { ElementOrder } from "../consts.js"

class CommonElements {
    static set_dist = `
        <p>Distribution</p>
        <select df-dist>
            <option value="exponential">Exponential</option>
            <option value="normal">Normal</option>
            <option value="erlang">Erlang</option>
            <option value="uniform">Uniform</option>
            <option value="constant">Constant</option>
        </select>
        <br>
    `

    static set_mean = `
        <p>Mean</p>
        <input type="number" value="1" min="0.001" df-mean>
        <br>
    `

    static set_deviation = `
        <p>Deviation</p>
        <input type="number" value="1" min="0.001" df-deviation>
        <br>
    `
    
    static set_replica_count = `
        <p>Replica count</p>
        <input type="number" df-replica value="1" min="1" step="1   ">
        <br>
    `
    
    static set_order = `
        <p>Element order</p>
        <select df-order>
            <option value="${ElementOrder.balanced}">Balanced</option>
            <option value="${ElementOrder.round_robin}">Round robin</option>
            <option value="${ElementOrder.random}">Random</option>
        </select>
        <br>
    `

    static set_order_for_single_io = `
        <select df-order hidden disabled>
            <option value="random">Random</option>
        </select>
    `
    
    static set_queue_size = `
        <p>Queue size</p>
        <input type="number" step="1" value="1" min="1" df-queuesize>
        <br>
    `

    static set_name(default_name) {
        return `
            <p>Name</p>
            <input class="df-name" type="text" value="${default_name}" df-name>
            <br>
        `
    }

    static modal(elements) {
        return `
            <div class="box dbclickbox" data-load="0">
                &nbsp;
                <div class="modal" style="display:none">
                    <div class="modal-content">
                        <span class="modal-close">&times;</span>
                        <br>
                        ${elements.join("")}
                    </div>
                </div>
            </div>
        `
    }
}


export const components = {
    "welcome": `
        <div>
            <div class="title-box"><i class="fa-solid fa-thumbtack"></i> Welcome!</div>
            <div class="box">
                <p><u><b>Values:</b></u></p>
                <p><b>Mean</b></p>
                <p>Depending on selected distribution it will set:</p>
                <ul>
                    <li>Exponential, normal, erlang: mean</li>
                    <li>Uniform: min</li>
                </ul>
                <p><b>Deviation</b></p>
                <p>Depending on selected distribution it will set:</p>
                <ul>
                    <li>Constant, exponential: nothing (can be left empty)</li>
                    <li>Normal: deviation</li>
                    <li>Erlang: k value</li>
                    <li>Uniform: max</li>
                </ul>
                <br><br>

                <p><u><b>Shortkeys:</b></u></p>
                <p><b>Delete</b> == Remove selected node</p>
                <p><b>Mouse Left Click</b> == Move</p>
                <p><b>Mouse Right</b> == Delete Option</p>
                <p><b>Ctrl + Wheel</b> == Zoom</p>
            </div>
        </div>
    `,

    "create": `
        <div>
            <div class="title-box">
                <i class="fa-solid fa-plus"></i> Create
            </div>
            ${CommonElements.modal([
                CommonElements.set_name("Create"),
                CommonElements.set_dist,
                CommonElements.set_mean,
                CommonElements.set_deviation,
                CommonElements.set_queue_size,
                CommonElements.set_order,
                CommonElements.set_replica_count
            ])}
        </div>
    `,

    "process": `
        <div>
            <div class="title-box">
                <i class="fa-solid fa-gears"></i> Process
            </div>
            ${CommonElements.modal([
                CommonElements.set_name("Process"),
                CommonElements.set_dist,
                CommonElements.set_mean,
                CommonElements.set_deviation,
                CommonElements.set_queue_size,
                CommonElements.set_order,
                CommonElements.set_replica_count
            ])}
        </div>
    `,

    "dispose": `
        <div>
            <div class="title-box">
                <i class="fa-solid fa-trash"></i> Dispose
            </div>
            ${CommonElements.modal([
                CommonElements.set_name("Dispose")
            ])}
        </div>
    `,

    "comment": `
        <div>
            <div class="title-box"><i class="fa-solid fa-comment"></i> Comment</div>
            <div class="box">
                <textarea class="textarea-comment" df-template></textarea>
            </div>
        </div>
    `,
}