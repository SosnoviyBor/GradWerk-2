export function showOverlay() {
    const bg = document.getElementById("overlay-bg")
    const text = document.getElementById("overlay-text")
    var i = 0
    const scale = 50

    const fadein = window.setInterval(() => {
        if (i > scale) {
            clearInterval(fadein);
            return
        }
        bg.style.opacity = i / (scale * 2);
        text.style.opacity = i / scale;
        i++;
    }, 10);
}


export function hideOverlay() {
    document.getElementById("sim-started-overlay").hidden = true
}