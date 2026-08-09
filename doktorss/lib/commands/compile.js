const fs = require("fs");

const { run } = require("../run");
const { DOKTORC_EXE } = require("../paths");

const { PREFIX } = require("../../data/prefix");

async function compile(inputFile) {
    if(!fs.existsSync(inputFile)) throw new Error(`${PREFIX} File not found: ${inputFile}.`);
    if(!fs.existsSync(DOKTORC_EXE)) throw new Error(`${PREFIX} doktorc.exe not found at ${DOKTORC_EXE}. Run "doktor update" first.`);

    console.log(`${PREFIX} Compiling ${inputFile}...`);
    await run(DOKTORC_EXE, [inputFile]);
    console.log(`${PREFIX} Compile command has been successfully executed.`)
}

module.exports = { compile };