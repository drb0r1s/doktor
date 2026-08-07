import { WebglRenderer } from "./renderers/webgl/webglRenderer.js";
import { TextRenderer } from "./renderers/textRenderer.js";
import { setupCanvas } from "./functions/setupCanvas.js";

export const Drawer = {
    webgl: async (numericBuffer, stringTable, drawStructuresCount) => {
        const canvas = setupCanvas("webgl-canvas");
    
        const webglRenderer = new WebglRenderer(canvas);
    
        await webglRenderer.preloadTextures(numericBuffer, stringTable, drawStructuresCount);
        webglRenderer.draw(numericBuffer, stringTable, drawStructuresCount);
    },

    text: (numericBuffer, stringTable, drawStructuresCount) => {
        const canvas = setupCanvas("text-canvas");
    
        const textRenderer = new TextRenderer(canvas);
        textRenderer.drawText(numericBuffer, stringTable, drawStructuresCount);
    },
};