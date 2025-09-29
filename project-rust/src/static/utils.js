export function postToNewTab(json, link) {
    // sanitization
    // json = clearNullTnexts(json)
    console.log(json)

    const inp = document.createElement("input");
    inp.type = "hidden";
    inp.name = "result";
    inp.value = JSON.stringify(json);

    const form = document.createElement("form");
    form.action = link;
    form.method = "post";
    form.target = "_blank";
    form.hidden = true;
    form.appendChild(inp);

    document.body.appendChild(form);
    form.submit();
    form.remove();
}

function clearNullTnexts(json) {
    for (const result of json.results) {
        for (const [k, v] of result.element.tnext.entries()) {
            if (v === null) {
                delete result.element.tnext[k]
            }
        }
    }
    return json
}

export function mean(arr) {
    return arr.reduce( ( p, c ) => p + c, 0 ) / arr.length
}