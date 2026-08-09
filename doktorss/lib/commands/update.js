const fs = require("fs");

const { run } = require("../run");
const { DOKTORC, DOKTORR, DOKTORC_OUT, DOKTORC_DEBUG_EXE, DOKTORC_EXE } = require("../paths");

async function update() {
    console.log("[DOKTOR Scripts] Updating DOKTORC and DOKTORR projects...");

    console.log("[DOKTOR Scripts] (1/3) Building doktorc (cargo build)...");

    await run("cargo", ["build"], { cwd: DOKTORC });

    console.log("[DOKTOR Scripts] (2/3) Moving doktorc.exe to doktorc/out/...");
    
    fs.mkdirSync(DOKTORC_OUT, { recursive: true });
    fs.copyFileSync(DOKTORC_DEBUG_EXE, DOKTORC_EXE);

    console.log("[DOKTOR Scripts] (3/3) Building doktorr (wasm-pack build --target web)...");

    await run("wasm-pack", ["build", "--target", "web"], { cwd: DOKTORR });

    console.log("[DOKTOR Scripts] Update command has been successfully executed.");
}

module.exports = { update };