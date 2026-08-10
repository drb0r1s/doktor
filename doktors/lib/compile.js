const fs = require("fs");
const { spawn } = require("child_process");

const { DOKTORC_EXE } = require("./paths");

function compile(doktorFile) {
    return new Promise((resolve, reject) => {
        if(!fs.existsSync(DOKTORC_EXE)) {
            reject(new Error(`doktorc binary not found at ${DOKTORC_EXE}. Run 'doktor update' first.`));
            return;
        }

        const child = spawn(DOKTORC_EXE, [doktorFile], {
            stdio: "inherit",
            shell: process.platform === "win32",
        });

        child.on("error", reject);

        child.on("exit", code => {
            if(code === 0) resolve();
            else reject(new Error(`doktorc exited with code ${code}`));
        });
    });
}

module.exports = { compile };
