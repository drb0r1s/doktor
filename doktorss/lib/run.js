const { spawn } = require("child_process");

function run(command, args, options = {}) {
    return new Promise((resolve, reject) => {
        const child = spawn(command, args, {
            stdio: "inherit",
            cwd: options.cwd,
            shell: process.platform === "win32",
        });

        child.on("error", reject);

        child.on("exit", code => {
            if(code === 0) resolve();
            else reject(new Error(`[DOKTOR Scripts] ${command} ${args.join(" ")} exited with code ${code}.`));
        });
    });
}

module.exports = { run };