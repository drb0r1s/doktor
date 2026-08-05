import init, { DoktorRuntime, getTextMeasurementRequests, getImageMeasurementRequests } from "../doktorr/pkg/doktorr.js";
import { WebglRenderer } from "./renderers/webgl/webglRenderer.js";
import { TextRenderer } from "./renderers/textRenderer.js";
import { measureTexts } from "./functions/measureTexts.js";
import { measureImages } from "./functions/measureImages.js";

async function run() {
    await init(); // Loads .wasm, has to be written first.

    const response = await fetch("../doktorc/src/out/compiled.doktorb");
    const bytes = new Uint8Array(await response.arrayBuffer());

    const textMeasurementRequests = getTextMeasurementRequests(bytes);
    const textMeasurements = measureTexts(textMeasurementRequests);

    const imageMeasurementRequests = getImageMeasurementRequests(bytes);
    const imageMeasurements = await measureImages(imageMeasurementRequests);

    const doktorRuntime = new DoktorRuntime();
    const parsed = doktorRuntime.compile(bytes, window.innerWidth, window.innerHeight, textMeasurements, imageMeasurements);

    const numericBuffer = parsed.numericBuffer();
    const stringTable = parsed.stringTable();

    const drawStructuresCount = numericBuffer.length / 16;

    await webglDraw(numericBuffer, stringTable, drawStructuresCount);
    textDraw(numericBuffer, stringTable, drawStructuresCount);
}

run().catch(console.error);

async function webglDraw(numericBuffer, stringTable, drawStructuresCount) {
    const canvas = getCanvas("webgl-canvas");

    const webglRenderer = new WebglRenderer(canvas);

    await webglRenderer.preloadTextures(numericBuffer, stringTable, drawStructuresCount);
    webglRenderer.draw(numericBuffer, stringTable, drawStructuresCount);
}

function textDraw(numericBuffer, stringTable, drawStructuresCount) {
    const canvas = getCanvas("text-canvas");

    const textRenderer = new TextRenderer(canvas);
    textRenderer.drawText(numericBuffer, stringTable, drawStructuresCount);
}

function getCanvas(id) {
    const canvas = document.getElementById(id);

    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;

    return canvas;
}