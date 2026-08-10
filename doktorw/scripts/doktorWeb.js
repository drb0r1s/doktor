import init, { DoktorRuntime, getTextMeasurementRequests, getImageMeasurementRequests } from "../../doktorr/pkg/doktorr.js";
import { Measurer } from "./measurer.js";
import { Drawer } from "./drawer.js";
import { Events } from "./events.js";

export class DoktorWeb {
    constructor() {
        this.doktorRuntime = null;
    }

    async begin() {
        try {
            await init(); // Loads .wasm, has to be written first.
        
            const response = await fetch("../doktorc/out/compiled.doktorb");
            const bytes = new Uint8Array(await response.arrayBuffer());
            
            const textMeasurements = await Measurer.texts(bytes);
            const imageMeasurements = await Measurer.images(bytes);
            
            this.doktorRuntime = new DoktorRuntime();
            return this.doktorRuntime.execute(bytes, window.innerWidth, window.innerHeight, textMeasurements, imageMeasurements);
        }

        catch(error) { console.error(error) }
    }

    async draw(executedDoktorRuntime = null) {
        try {
            if(executedDoktorRuntime === null) return;

            const numericBuffer = executedDoktorRuntime.numericBuffer();
            const stringTable = executedDoktorRuntime.stringTable();
            
            const drawStructuresCount = numericBuffer.length / 16;
                
            await Drawer.webgl(numericBuffer, stringTable, drawStructuresCount);
            Drawer.text(numericBuffer, stringTable, drawStructuresCount);
        }

        catch(error) { console.error(error) }
    }

    setupEvents() {
        if(this.doktorRuntime === null) return;

        Events.click(this.doktorRuntime);
        Events.scroll(this.doktorRuntime);
    }
}