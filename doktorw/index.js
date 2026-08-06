import { DoktorWeb } from "./scripts/doktorWeb.js";

const doktorWeb = new DoktorWeb();

async function run() {
    const compiledDoktorRuntime = await doktorWeb.compile();
    await redraw(compiledDoktorRuntime);

    doktorWeb.setupEvents();
}

export async function redraw(compiledDoktorRuntime) {
    await doktorWeb.draw(compiledDoktorRuntime);
}

run();