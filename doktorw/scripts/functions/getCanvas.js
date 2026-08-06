export function getCanvas(id) {
    const canvas = document.getElementById(id);

    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;

    return canvas;
}