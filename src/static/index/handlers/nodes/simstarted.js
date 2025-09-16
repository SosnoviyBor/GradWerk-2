
export function showOverlay() {
    const overlay = document.getElementById("sim-started-overlay");
    overlay.classList.remove("fade-out");
    overlay.style.opacity = "1";
    overlay.hidden = false;
}


export function hideOverlay() {
    const overlay = document.getElementById("sim-started-overlay");
    overlay.classList.add("fade-out");
    // Hide after animation completes (220ms)
    setTimeout(() => {
        overlay.hidden = true;
        overlay.classList.remove("fade-out");
    }, 230);
}