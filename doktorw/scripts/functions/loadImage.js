export function loadImage(source) {
    return new Promise((resolve, reject) => {
        const img = new window.Image();

        img.onload = () => resolve(img);
        img.onerror = () => reject(new Error(`Failed to load image: ${source}`));

        img.src = source;
    });
}