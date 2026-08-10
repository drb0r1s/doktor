#!/usr/bin/env node

const path = require("path");
const { startServer } = require("../lib/server");

async function main() {
    const [doktorFile] = process.argv.slice(2);

    if(!doktorFile) {
        console.error("Usage: doktors <file.doktor>");
        process.exit(1);
    }

    await startServer(path.resolve(process.cwd(), doktorFile));
}

main().catch(error => {
    console.error(error.message ?? error);
    process.exit(1);
});