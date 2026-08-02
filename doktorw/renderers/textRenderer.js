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

            const borderTopColor = unpackColor(numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BORDER_TOP_COLOR]);
            const borderTopColorAlpha = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BORDER_TOP_COLOR_ALPHA];
            const borderTopSize = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BORDER_TOP_SIZE];
            const borderTopType = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BORDER_TOP_TYPE];

            const borderBottomColor = unpackColor(numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BORDER_BOTTOM_COLOR]);
            const borderBottomColorAlpha = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BORDER_BOTTOM_COLOR_ALPHA];
            const borderBottomSize = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BORDER_BOTTOM_SIZE];
            const borderBottomType = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BORDER_BOTTOM_TYPE];

            const borderLeftColor = unpackColor(numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BORDER_LEFT_COLOR]);
            const borderLeftColorAlpha = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BORDER_LEFT_COLOR_ALPHA];
            const borderLeftSize = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BORDER_LEFT_SIZE];
            const borderLeftType = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BORDER_LEFT_TYPE];

            const borderRightColor = unpackColor(numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BORDER_RIGHT_COLOR]);
            const borderRightColorAlpha = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BORDER_RIGHT_COLOR_ALPHA];
            const borderRightSize = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BORDER_RIGHT_SIZE];
            const borderRightType = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BORDER_RIGHT_TYPE];

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

            this.drawBorder(x, y, textWidth, 0, borderTopColor, borderTopColorAlpha, borderTopSize, borderTopType);
            this.drawBorder(x, y + textHeight, textWidth, 0, borderBottomColor, borderBottomColorAlpha, borderBottomSize, borderBottomType, true);
            this.drawBorder(x, y, 0, textHeight, borderLeftColor, borderLeftColorAlpha, borderLeftSize, borderLeftType);
            this.drawBorder(x + textWidth, y, 0, textHeight, borderRightColor, borderRightColorAlpha, borderRightSize, borderRightType);

            context.fillStyle = `rgba(${contentColor.r}, ${contentColor.g}, ${contentColor.b}, ${contentColorAlpha / 255})`;
            context.fillText(content, x, y);

            context.globalAlpha = 1.0;
            context.restore();
        }
    }

    drawBorder(startX, startY, endX, endY, color, colorAlpha, size, type) {
        if(size <= 0) return;

        const context = this.context;

        context.strokeStyle = `rgba(${color.r}, ${color.g}, ${color.b}, ${colorAlpha / 255})`;
        context.lineWidth = size;

        if(type === BORDER_TYPES.DASHED) context.setLineDash([12, 8]);
        else if(type === BORDER_TYPES.DOTTED) context.setLineDash([2, 4]);
        else context.setLineDash([]);

        context.beginPath();
        context.moveTo(startX, startY);
        context.lineTo(startX + endX, startY + endY);
        context.stroke();
    }
}