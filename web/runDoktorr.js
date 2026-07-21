import init, { parseDoktorb } from "../doktorr/pkg/doktorr.js";

async function run() {
    await init(); // Loads .wasm, has to be written first.

    const response = await fetch("../doktorc/src/out/compiled.doktorb");
    const bytes = new Uint8Array(await response.arrayBuffer());

    const parsed = parseDoktorb(bytes);

    console.log(parsed.numericBuffer());
    console.log(parsed.stringTable());
}

run().catch(console.error);