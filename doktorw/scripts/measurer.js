import { getTextMeasurementRequests, getImageMeasurementRequests } from "../../doktorr/pkg/doktorr.js";
import { wrapByWord } from "./functions/wrapByWord.js";
import { loadImage } from "./functions/loadImage.js";

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

        if(request.width && request.width > 0) {
            const lines = wrapByWord(measureContext, request.content, request.width);
            const lineMetrics = measureContext.measureText("A"); // "A" here is just for reference, it could be any character.
            const lineHeight = lineMetrics.fontBoundingBoxAscent + lineMetrics.fontBoundingBoxDescent;

            return {
                path: request.path,
                width: request.width,
                height: lineHeight * lines.length,
                lines,
            };
        }

        const metrics = measureContext.measureText(request.content);

        return {
            path: request.path,
            width: metrics.width,
            height: metrics.fontBoundingBoxAscent + metrics.fontBoundingBoxDescent,
            lines: [request.content],
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