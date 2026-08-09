const fs = require("fs");

const { run } = require("../run");
const { DOKTORC_EXE } = require("../paths");

async function compile(inputFile) {
    if(!fs.existsSync(inputFile)) {
        throw new Error(`Input file not found: ${inputFile}`);
    }

    if(!fs.existsSync(DOKTORC_EXE)) throw new Error(`doktorc executable not found at ${DOKTORC_EXE}. Run 'doktor update' first.`);

    console.log(`Compiling ${inputFile}...`);
    await run(DOKTORC_EXE, [inputFile]);
    console.log("Compiled to doktorc/out/compiled.doktorb");
}

module.exports = { compile };