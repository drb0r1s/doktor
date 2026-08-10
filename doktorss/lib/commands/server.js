const path = require("path");
const { run } = require("../run");

// Here we need to establish the actual bridge between DOKTORSS and DOKTORS in order to use the server script.
const DOKTORS_JS = path.resolve(__dirname, "..", "..", "..", "doktors", "bin", "doktors.js");

async function server(doktorFile) {
    await run("node", [DOKTORS_JS, doktorFile]);
}

module.exports = { server };