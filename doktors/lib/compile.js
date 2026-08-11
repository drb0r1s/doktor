const fs = require("fs");
const { spawn } = require("child_process");

const { DOKTORC_EXE } = require("./paths");
const { PREFIX } = require("../data/prefix");

function compile(doktorFile) {
    return new Promise((resolve, reject) => {
        if(!fs.existsSync(DOKTORC_EXE)) {
            reject(new Error(`doktorc.exe not found at ${DOKTORC_EXE}. Run "doktor update" first.`));
            return;
        }

        const child = spawn(DOKTORC_EXE, [doktorFile], {
            stdio: "inherit",
            shell: process.platform === "win32",
        });

        child.on("error", reject);

        child.on("exit", code => {
            if(code === 0) resolve();
            else reject(new Error(`doktorc exited with code ${code}.`));
        });
    });
}

module.exports = { compile };
