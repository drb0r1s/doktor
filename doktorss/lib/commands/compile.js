const fs = require("fs");

const { run } = require("../run");
const { DOKTORC_EXE } = require("../paths");

const { prefix } = require("../../data/prefix");

async function compile(inputFile) {
    if(!fs.existsSync(inputFile)) throw new Error(`${prefix} File not found: ${inputFile}.`);
    if(!fs.existsSync(DOKTORC_EXE)) throw new Error(`${prefix} doktorc.exe not found at ${DOKTORC_EXE}. Run "doktor update" first.`);

    console.log(`${prefix} Compiling ${inputFile}...`);
    await run(DOKTORC_EXE, [inputFile]);
    console.log(`${prefix} Compile command has been successfully executed.`)
}

module.exports = { compile };