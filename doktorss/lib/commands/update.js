const fs = require("fs");

const { run } = require("../run");
const { DOKTORC, DOKTORR, DOKTORC_OUT, DOKTORC_DEBUG_EXE, DOKTORC_EXE } = require("../paths");

async function update() {
    console.log("[1/3] Building doktorc (cargo build)...");

    await run("cargo", ["build"], { cwd: DOKTORC });

    console.log("[2/3] Moving doktorc binary to doktorc/out/...");
    
    fs.mkdirSync(DOKTORC_OUT, { recursive: true });
    fs.copyFileSync(DOKTORC_DEBUG_EXE, DOKTORC_EXE);

    console.log("[3/3] Building doktorr (wasm-pack build --target web)...");

    await run("wasm-pack", ["build", "--target", "web"], { cwd: DOKTORR });

    console.log("Update complete.");
}

module.exports = { update };