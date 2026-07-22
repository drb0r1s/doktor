import init, { parseDoktorb } from "../doktorr/pkg/doktorr.js";
import { WebglRenderer } from "./webglRenderer.js";

async function run() {
    await init(); // Loads .wasm, has to be written first.

    const response = await fetch("../doktorc/src/out/compiled.doktorb");
    const bytes = new Uint8Array(await response.arrayBuffer());

    const parsed = parseDoktorb(bytes);

    const numericBuffer = parsed.numericBuffer();
    const drawStructuresCount = numericBuffer.length / 16;

    webglDraw(numericBuffer, drawStructuresCount);
}

run().catch(console.error);

function webglDraw(numericBuffer, drawStructuresCount) {
    const canvas = document.getElementById("webgl-canvas");

    const webglRenderer = new WebglRenderer(canvas);
    webglRenderer.drawRectangles(numericBuffer, drawStructuresCount);
}