import { unpackColor } from "../functions/unpackColor.js";
import { getFont } from "../functions/getFont.js";
import { PACKET_STRUCTURE } from "../data/packetStructure.js";

export class TextRenderer {
    constructor(canvas) {
        this.canvas = canvas;
        this.context = canvas.getContext("2d");
    }

    drawText(numericBuffer, stringTable, drawStructuresCount) {
        const context = this.context;

        context.clearRect(0, 0, this.canvas.width, this.canvas.height);
        context.textBaseline = "top"; // Aligning the starting point with Shaper.

        const decoder = new TextDecoder("utf-8");

        for (let i = 0; i < drawStructuresCount; i++) {
            const rowStart = i * PACKET_STRUCTURE.PACKET_SIZE;
            const type = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_TYPE];

            if (type !== PACKET_STRUCTURE.PACKET_TEXT_TYPE) continue;

            const x = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_X];
            const y = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_Y];

            const offset = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_STRING_OFFSET];
            const length = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_STRING_LENGTH];

            const { r, g, b } = unpackColor(numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_CONTENT_COLOR]);

            const contentSize = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_CONTENT_SIZE];
            const contentFont = getFont(numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_CONTENT_FONT]);
            console.log(contentFont, numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_CONTENT_FONT])

            const content = decoder.decode(stringTable.subarray(offset, offset + length));

            context.font = `${contentSize}px ${contentFont}`;
            context.fillStyle = `rgb(${r}, ${g}, ${b})`;

            context.fillText(content, x, y);
        }
    }
}