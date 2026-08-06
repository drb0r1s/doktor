import { getTextMeasurementRequests, getImageMeasurementRequests } from "../../doktorr/pkg/doktorr.js";

export const Measurer = {
    texts: bytes => {
        const textMeasurementRequests = getTextMeasurementRequests(bytes);
        return measureTexts(textMeasurementRequests);
    },

    images: async bytes => {
        const imageMeasurementRequests = getImageMeasurementRequests(bytes);
        return await measureImages(imageMeasurementRequests);
    },
};

function measureTexts(textMeasurementRequests) {
    const measureCanvas = document.createElement("canvas");
    const measureContext = measureCanvas.getContext("2d");

    measureContext.textBaseline = "top";

    return textMeasurementRequests.map(request => {
        measureContext.font = `${request.content_size}px ${request.content_font}`;

        const metrics = measureContext.measureText(request.content);

        return {
            path: request.path,
            width: metrics.width,
            height: metrics.fontBoundingBoxAscent + metrics.fontBoundingBoxDescent,
        };
    });
}

async function measureImages(imageMeasurementRequests) {
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