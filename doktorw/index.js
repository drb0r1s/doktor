import { DoktorWeb } from "./scripts/doktorWeb.js";

const doktorWeb = new DoktorWeb();

async function run() {
    const executedDoktorRuntime = await doktorWeb.begin();
    await redraw(executedDoktorRuntime);

    doktorWeb.setupEvents();
}

export async function redraw(executedDoktorRuntime) {
    await doktorWeb.draw(executedDoktorRuntime);
}

run();