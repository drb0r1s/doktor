export async function measureImages(imageMeasurementRequests) {
    return Promise.all(
        imageMeasurementRequests.map(async request => {
            const image = await loadImage(request.source);

            return {
                path: request.path,
                width: image.naturalWidth,
                height: image.naturalHeight,
            };
        })
    );
}

function loadImage(source) {
    return new Promise((resolve, reject) => {
        const image = new window.Image();

        image.onload = () => resolve(image);
        image.onerror = () => reject(new Error(`Failed to load image: ${source}`));

        image.src = source;
    });
}