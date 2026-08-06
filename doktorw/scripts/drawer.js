import { WebglRenderer } from "./renderers/webgl/webglRenderer.js";
import { TextRenderer } from "./renderers/textRenderer.js";
import { getCanvas } from "./functions/getCanvas.js";

export const Drawer = {
    webgl: async (numericBuffer, stringTable, drawStructuresCount) => {
        const canvas = getCanvas("webgl-canvas");
    
        const webglRenderer = new WebglRenderer(canvas);
    
        await webglRenderer.preloadTextures(numericBuffer, stringTable, drawStructuresCount);
        webglRenderer.draw(numericBuffer, stringTable, drawStructuresCount);
    },

    text: (numericBuffer, stringTable, drawStructuresCount) => {
        const canvas = getCanvas("text-canvas");
    
        const textRenderer = new TextRenderer(canvas);
        textRenderer.drawText(numericBuffer, stringTable, drawStructuresCount);
    },
};