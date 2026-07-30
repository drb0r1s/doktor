import { unpackColor } from "../functions/unpackColor.js";
import { getFont } from "../functions/getFont.js";
import { PACKET_STRUCTURE } from "../data/packetStructure.js";
import { BORDER_TYPES } from "../data/borderTypes.js";

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

        for(let i = 0; i < drawStructuresCount; i++) {
            const rowStart = i * PACKET_STRUCTURE.PACKET_SIZE;
            const type = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_TYPE];

            if(type !== PACKET_STRUCTURE.PACKET_TEXT_TYPE) continue;

            const x = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_X];
            const y = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_Y];

            const clipXStart = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_CLIP_X_START];
            const clipXEnd = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_CLIP_X_END];
            const clipYStart = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_CLIP_Y_START];
            const clipYEnd = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_CLIP_Y_END];

            const backgroundColor = unpackColor(numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BACKGROUND_COLOR]);
            const backgroundColorAlpha = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BACKGROUND_COLOR_ALPHA];

            const offset = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_STRING_OFFSET];
            const length = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_STRING_LENGTH];

            const contentColor = unpackColor(numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_CONTENT_COLOR]);
            const contentColorAlpha = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_CONTENT_COLOR_ALPHA];

            const contentSize = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_CONTENT_SIZE];
            const contentFont = getFont(numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_CONTENT_FONT]);

            context.font = `${contentSize}px ${contentFont}`;

            const borderColor = unpackColor(numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BORDER_COLOR]);
            const borderColorAlpha = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BORDER_COLOR_ALPHA];

            const borderSize = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BORDER_SIZE];
            const borderType = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BORDER_TYPE];

            const opacity = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_OPACITY];

            const content = decoder.decode(stringTable.subarray(offset, offset + length));

            const metrics = context.measureText(content);

            const textWidth = metrics.width;
            const textHeight = metrics.fontBoundingBoxAscent + metrics.fontBoundingBoxDescent;

            context.save();
            context.beginPath();
            context.rect(clipXStart, clipYStart, Math.max(0, clipXEnd - clipXStart), Math.max(0, clipYEnd - clipYStart));
            context.clip();

            context.globalAlpha = opacity;

            context.fillStyle = `rgba(${backgroundColor.r}, ${backgroundColor.g}, ${backgroundColor.b}, ${backgroundColorAlpha / 255})`;
            context.fillRect(x, y, textWidth, textHeight);

            if(borderSize > 0) {
                context.strokeStyle = `rgba(${borderColor.r}, ${borderColor.g}, ${borderColor.b}, ${borderColorAlpha / 255})`;
                context.lineWidth = borderSize;

                if(borderType === BORDER_TYPES.DASHED) context.setLineDash([12, 8]);
                else if (borderType === BORDER_TYPES.DOTTED) context.setLineDash([2, 4]);
                else context.setLineDash([]);

                context.strokeRect(x + borderSize / 2, y + borderSize / 2, textWidth - borderSize, textHeight - borderSize);
            }

            context.fillStyle = `rgba(${contentColor.r}, ${contentColor.g}, ${contentColor.b}, ${contentColorAlpha / 255})`;
            context.fillText(content, x, y);

            context.globalAlpha = 1.0;
            context.restore();
        }
    }
}