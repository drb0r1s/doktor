import { redraw } from "../index.js";
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
    },

    scroll: doktorRuntime => {
        const textCanvas = getCanvas("text-canvas");

        textCanvas.addEventListener("wheel", event => {
            const rect = textCanvas.getBoundingClientRect();

            const x = event.clientX - rect.left;
            const y = event.clientY - rect.top;

            const scrollableBlock = doktorRuntime.getScrollableAncestor(x, y);

            if(!scrollableBlock) return;

            event.preventDefault();

            const currentOffset = scrollableBlock.scrollOffset ?? { x: 0, y: 0 };

            const newX = currentOffset.x + event.deltaX;
            const newY = currentOffset.y + event.deltaY;

            const compiledDoktorRuntime = doktorRuntime.updateScrollOffset(scrollableBlock.id, newX, newY);
            redraw(compiledDoktorRuntime);
        });
    },
};