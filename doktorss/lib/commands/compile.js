const fs = require("fs");

const { run } = require("../run");
const { DOKTORC_EXE } = require("../paths");

async function compile(inputFile) {
    if(!fs.existsSync(inputFile)) throw new Error(`[DOKTOR Scripts] File not found: ${inputFile}.`);
    if(!fs.existsSync(DOKTORC_EXE)) throw new Error(`[DOKTOR Scripts] doktorc.exe not found at ${DOKTORC_EXE}. Run "doktor update" first.`);

    console.log(`[DOKTOR Scripts] Compiling ${inputFile}...`);
    await run(DOKTORC_EXE, [inputFile]);
    console.log(`[DOKTOR Scripts] Compile command has been successfully executed.`)
}

module.exports = { compile };