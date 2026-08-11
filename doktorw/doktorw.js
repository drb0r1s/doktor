import { DoktorWeb } from "./scripts/doktorWeb.js";

const doktorWeb = new DoktorWeb();

async function run() {
    console.log(
        "%c[%cDOKTOR%c Web]%c Beginning the execution of %cDOKTOR Runtime%c...",
        "color: cyan;",
        "color: cyan; font-weight: bold;",
        "color: cyan;",
        "color: inherited;",
        "color: orange; font-style: italic;",
        "color: inherited;",
    );

    const executedDoktorRuntime = await doktorWeb.begin();

    console.log(
        "%c[%cDOKTOR%c Web]%c DOKTOR Runtime%c has been executed.",
        "color: cyan;",
        "color: cyan; font-weight: bold;",
        "color: cyan;",
        "color: orange; font-style: italic;",
        "color: inherited;",
    );

    await redraw(executedDoktorRuntime);

    console.log(
        "%c[%cDOKTOR%c Web]%c Drawing structures have been drawn.",
        "color: cyan;",
        "color: cyan; font-weight: bold;",
        "color: cyan;",
        "color: inherited;",
    );

    doktorWeb.setupEvents();

    console.log(
        "%c[%cDOKTOR%c Web]%c Events have been activated.",
        "color: cyan;",
        "color: cyan; font-weight: bold;",
        "color: cyan;",
        "color: inherited;",
    );
}

export async function redraw(executedDoktorRuntime) {
    await doktorWeb.draw(executedDoktorRuntime);
}

run();