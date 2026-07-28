export function measureTexts(textMeasurementRequests) {
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