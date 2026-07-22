import init, { parseDoktorb } from "../doktorr/pkg/doktorr.js";
import { WebglRenderer } from "./webglRenderer.js";
import { TextRenderer } from "./textRenderer.js";

async function run() {
    await init(); // Loads .wasm, has to be written first.

    const response = await fetch("../doktorc/src/out/compiled.doktorb");
    const bytes = new Uint8Array(await response.arrayBuffer());

    const parsed = parseDoktorb(bytes);

    const numericBuffer = parsed.numericBuffer();
    const stringTable = parsed.stringTable();

    const drawStructuresCount = numericBuffer.length / 16;

    webglDraw(numericBuffer, drawStructuresCount);
    textDraw(numericBuffer, stringTable, drawStructuresCount);
}

run().catch(console.error);

function webglDraw(numericBuffer, drawStructuresCount) {
    const canvas = document.getElementById("webgl-canvas");

    const webglRenderer = new WebglRenderer(canvas);
    webglRenderer.drawRectangles(numericBuffer, drawStructuresCount);
}

function textDraw(numericBuffer, stringTable, drawStructuresCount) {
    const canvas = document.getElementById("text-canvas");

    const textRenderer = new TextRenderer(canvas);
    textRenderer.drawText(numericBuffer, stringTable, drawStructuresCount);
}