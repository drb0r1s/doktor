import { getCanvas } from "./functions/getCanvas.js";

export const Events = {
    click: doktorRuntime => {
        const textCanvas = getCanvas("text-canvas"); // Text canvas is located over the WebGL one, so it will get all the events.

        textCanvas.addEventListener("click", event => {
            const rect = textCanvas.getBoundingClientRect();

            const x = event.clientX - rect.left;
            const y = event.clientY - rect.top;

            const block = doktorRuntime.getBlock(x, y);

            if(block) console.log(block);
        });
    }
};