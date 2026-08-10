import { redraw } from "../index.js";

export const Events = {
    click: doktorRuntime => {
        const textCanvas = document.getElementById("text-canvas"); // Text canvas is located over the WebGL one, so it will get all the events.

        textCanvas.addEventListener("click", event => {
            const rect = textCanvas.getBoundingClientRect();

            const x = event.clientX - rect.left;
            const y = event.clientY - rect.top;

            const block = doktorRuntime.getBlock(x, y);

            if(block) console.log(block);
        });
    },

    scroll: doktorRuntime => {
        const textCanvas = document.getElementById("text-canvas");
        
        const scrollOffsets = new Map();
        let pendingOffset = null;

        textCanvas.addEventListener("wheel", async event => {
            const rect = textCanvas.getBoundingClientRect();

            const x = event.clientX - rect.left;
            const y = event.clientY - rect.top;

            const scrollableBlock = doktorRuntime.getScrollableAncestor(x, y);

            if(!scrollableBlock) return;

            event.preventDefault();

            const currentOffset = scrollOffsets.get(scrollableBlock.id) ?? { x: 0, y: 0 };

            const newOffset = {
                x: currentOffset.x + event.deltaX,
                y: currentOffset.y + event.deltaY,
            };

            scrollOffsets.set(scrollableBlock.id, newOffset);
            pendingOffset = { id: scrollableBlock.id, x: newOffset.x, y: newOffset.y };
        });

        requestAnimationFrame(tick);

        async function tick() {
            if(pendingOffset) {
                const { id, x, y } = pendingOffset;
                pendingOffset = null;

                const executedDoktorRuntime = doktorRuntime.updateScrollOffset(id, x, y);
                await redraw(executedDoktorRuntime);
            }

            requestAnimationFrame(tick);
        }
    },
};