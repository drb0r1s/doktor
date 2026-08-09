#!/usr/bin/env node

const path = require("path");

const { compile } = require("../lib/commands/compile");
const { update } = require("../lib/commands/update");

const { prefix } = require("../data/prefix");

async function main() {
    const [command, ...args] = process.argv.slice(2);

    switch(command) {
        case "compile": {
            const inputFile = args[0];

            if (!inputFile) {
                console.error(`${prefix} Usage: doktor compile <file.doktor>.`);
                process.exit(1);
            }

            await compile(path.resolve(process.cwd(), inputFile));
            break;
        }

        case "update": {
            await update();
            break;
        }

        default: {
            console.error(`${prefix} Unknown command: ${command ?? "(empty)"}.`);
            console.error(`${prefix} Available commands: compile <file_name.doktor>, update.`);

            process.exit(1);
        }
    }
}

main().catch(error => {
    console.error(`${prefix} ${error.message ?? error}`);
    process.exit(1);
});