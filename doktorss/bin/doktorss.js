#!/usr/bin/env node

const path = require("path");

const { compile } = require("../lib/commands/compile");
const { update } = require("../lib/commands/update");
const { server } = require("../lib/commands/server");

const { PREFIX } = require("../data/prefix");

async function main() {
    const [command, ...args] = process.argv.slice(2);

    switch(command) {
        case "compile": {
            const inputFile = args[0];

            if(!inputFile) {
                console.error(`${PREFIX} Usage: doktor compile <file_name.doktor>.`);
                process.exit(1);
            }

            await compile(path.resolve(process.cwd(), inputFile));
            break;
        }

        case "update": {
            await update();
            break;
        }

        case "server": {
            const doktorFile = args[0];

            if(!doktorFile) {
                console.error(`${PREFIX} Usage: doktor server <file_name.doktor>`);
                process.exit(1);
            }

            await server(path.resolve(process.cwd(), doktorFile));
            break;
        }

        default: {
            console.error(`${PREFIX} Unknown command: ${command ?? "(empty)"}.`);
            console.error(`${PREFIX} Available commands: compile <file_name.doktor>, update.`);

            process.exit(1);
        }
    }
}

main().catch(error => {
    console.error(`${PREFIX} ${error.message ?? error}`);
    process.exit(1);
});