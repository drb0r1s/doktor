#!/usr/bin/env node

const path = require("path");
const { startServer } = require("../lib/server");

const { PREFIX } = require("../data/prefix");

async function main() {
    const [doktorFile] = process.argv.slice(2);

    if(!doktorFile) {
        console.error(`${PREFIX} Usage: doktors <file.doktor>`);
        process.exit(1);
    }

    await startServer(path.resolve(process.cwd(), doktorFile));
}

main().catch(error => {
    console.error(`${PREFIX} ${error.message ?? error}`);
    process.exit(1);
});