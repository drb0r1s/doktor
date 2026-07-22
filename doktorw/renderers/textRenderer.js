const PACKET_SIZE = 16;

const PACKET_TYPE = 0;
const PACKET_X = 1;
const PACKET_Y = 2;
const PACKET_WIDTH_OR_FONT_SIZE = 3;
const PACKET_R = 5;
const PACKET_G = 6;
const PACKET_B = 7;
const PACKET_STRING_OFFSET = 8;
const PACKET_STRING_LENGTH = 9;

const PACKET_TEXT_TYPE = 1;

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
            const rowStart = i * PACKET_SIZE;
            const type = numericBuffer[rowStart + PACKET_TYPE];

            if (type !== PACKET_TEXT_TYPE) continue;

            const x = numericBuffer[rowStart + PACKET_X];
            const y = numericBuffer[rowStart + PACKET_Y];

            const fontSize = numericBuffer[rowStart + PACKET_WIDTH_OR_FONT_SIZE];

            const r = numericBuffer[rowStart + PACKET_R];
            const g = numericBuffer[rowStart + PACKET_G];
            const b = numericBuffer[rowStart + PACKET_B];

            const offset = numericBuffer[rowStart + PACKET_STRING_OFFSET];
            const length = numericBuffer[rowStart + PACKET_STRING_LENGTH];

            const content = decoder.decode(stringTable.subarray(offset, offset + length));

            context.font = `${fontSize}px sans-serif`;
            context.fillStyle = `rgb(${r}, ${g}, ${b})`;

            context.fillText(content, x, y);
        }
    }
}