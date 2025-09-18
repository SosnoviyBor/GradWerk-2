export function updateName(ev) {
    const titleBox = ev.target.closest(".box").previousElementSibling
    const boxHTML = titleBox.innerHTML.split("</i>")
    boxHTML[1] = " " + ev.target.value
    titleBox.innerHTML = boxHTML.join("</i>")
}