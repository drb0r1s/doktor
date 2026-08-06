import { DoktorWeb } from "./scripts/doktorWeb.js";

async function run() {
    const doktorWeb = new DoktorWeb();

    await doktorWeb.compile();
    await doktorWeb.draw();
    doktorWeb.setupEvents();
}

run();