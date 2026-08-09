import { unpackColor } from "../../functions/unpackColor.js";
import { loadImage } from "../../functions/loadImage.js";
import { RECTANGLE_VERTEX_SHADER_SOURCE, RECTANGLE_FRAGMENT_SHADER_SOURCE } from "./shaders/rectangle.js";
import { IMAGE_VERTEX_SHADER_SOURCE, IMAGE_FRAGMENT_SHADER_SOURCE } from "./shaders/image.js";
import { PACKET_STRUCTURE } from "../../data/packetStructure.js";

export class WebglRenderer {
    constructor(canvas) {
        const gl = canvas.getContext("webgl");

        if(!gl) throw new Error("WebGL not supported on this canvas");

        this.gl = gl;
        this.canvas = canvas;

        // Rectangle Program
        this.rectangleProgram = createProgram(gl, RECTANGLE_VERTEX_SHADER_SOURCE, RECTANGLE_FRAGMENT_SHADER_SOURCE);
        this.rectanglePositionLocation = gl.getAttribLocation(this.rectangleProgram, "a_position");
        this.rectangleLocalPositionLocation = gl.getAttribLocation(this.rectangleProgram, "a_localPosition");
        this.rectangleResolutionLocation = gl.getUniformLocation(this.rectangleProgram, "u_resolution");
        this.rectangleFillColorLocation = gl.getUniformLocation(this.rectangleProgram, "u_fillColor");
        this.rectangleRectSizeLocation = gl.getUniformLocation(this.rectangleProgram, "u_rectSize");
        this.rectangleOpacityLocation = gl.getUniformLocation(this.rectangleProgram, "u_opacity");

        this.rectangleBorderTopColorLocation = gl.getUniformLocation(this.rectangleProgram, "u_borderTopColor");
        this.rectangleBorderTopSizeLocation = gl.getUniformLocation(this.rectangleProgram, "u_borderTopSize");
        this.rectangleBorderTopTypeLocation = gl.getUniformLocation(this.rectangleProgram, "u_borderTopType");

        this.rectangleBorderBottomColorLocation = gl.getUniformLocation(this.rectangleProgram, "u_borderBottomColor");
        this.rectangleBorderBottomSizeLocation = gl.getUniformLocation(this.rectangleProgram, "u_borderBottomSize");
        this.rectangleBorderBottomTypeLocation = gl.getUniformLocation(this.rectangleProgram, "u_borderBottomType");

        this.rectangleBorderLeftColorLocation = gl.getUniformLocation(this.rectangleProgram, "u_borderLeftColor");
        this.rectangleBorderLeftSizeLocation = gl.getUniformLocation(this.rectangleProgram, "u_borderLeftSize");
        this.rectangleBorderLeftTypeLocation = gl.getUniformLocation(this.rectangleProgram, "u_borderLeftType");

        this.rectangleBorderRightColorLocation = gl.getUniformLocation(this.rectangleProgram, "u_borderRightColor");
        this.rectangleBorderRightSizeLocation = gl.getUniformLocation(this.rectangleProgram, "u_borderRightSize");
        this.rectangleBorderRightTypeLocation = gl.getUniformLocation(this.rectangleProgram, "u_borderRightType");

        // Image Program
        this.imageProgram = createProgram(gl, IMAGE_VERTEX_SHADER_SOURCE, IMAGE_FRAGMENT_SHADER_SOURCE);
        this.imagePositionLocation = gl.getAttribLocation(this.imageProgram, "a_position");
        this.imageTexCoordLocation = gl.getAttribLocation(this.imageProgram, "a_texCoord");
        this.imageResolutionLocation = gl.getUniformLocation(this.imageProgram, "u_resolution");
        this.imageOpacityLocation = gl.getUniformLocation(this.imageProgram, "u_opacity");
        this.imageLocalPositionLocation = gl.getAttribLocation(this.imageProgram, "a_localPosition");
        this.imageBackgroundColorLocation = gl.getUniformLocation(this.imageProgram, "u_backgroundColor");
        this.imageRectSizeLocation = gl.getUniformLocation(this.imageProgram, "u_rectSize");

        this.imageBorderTopColorLocation = gl.getUniformLocation(this.imageProgram, "u_borderTopColor");
        this.imageBorderTopSizeLocation = gl.getUniformLocation(this.imageProgram, "u_borderTopSize");
        this.imageBorderTopTypeLocation = gl.getUniformLocation(this.imageProgram, "u_borderTopType");

        this.imageBorderBottomColorLocation = gl.getUniformLocation(this.imageProgram, "u_borderBottomColor");
        this.imageBorderBottomSizeLocation = gl.getUniformLocation(this.imageProgram, "u_borderBottomSize");
        this.imageBorderBottomTypeLocation = gl.getUniformLocation(this.imageProgram, "u_borderBottomType");

        this.imageBorderLeftColorLocation = gl.getUniformLocation(this.imageProgram, "u_borderLeftColor");
        this.imageBorderLeftSizeLocation = gl.getUniformLocation(this.imageProgram, "u_borderLeftSize");
        this.imageBorderLeftTypeLocation = gl.getUniformLocation(this.imageProgram, "u_borderLeftType");

        this.imageBorderRightColorLocation = gl.getUniformLocation(this.imageProgram, "u_borderRightColor");
        this.imageBorderRightSizeLocation = gl.getUniformLocation(this.imageProgram, "u_borderRightSize");
        this.imageBorderRightTypeLocation = gl.getUniformLocation(this.imageProgram, "u_borderRightType");

        // Shared Position Buffer
        this.positionBuffer = gl.createBuffer();
        this.localPositionBuffer = gl.createBuffer();
        this.texCoordBuffer = gl.createBuffer();

        this.textureCache = new Map(); // Image source -> WebGL texture
    }

    // Called once before drawing, so that textures are ready before drawing Image blocks.
    async preloadTextures(numericBuffer, stringTable, drawStructuresCount) {
        const decoder = new TextDecoder("utf-8");
        const sources = new Set();

        for(let i = 0; i < drawStructuresCount; i++) {
            const rowStart = i * PACKET_STRUCTURE.PACKET_SIZE;
            if(numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_TYPE] !== PACKET_STRUCTURE.PACKET_IMAGE_TYPE) continue;

            const offset = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_STRING_OFFSET];
            const length = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_STRING_LENGTH];

            sources.add(decoder.decode(stringTable.subarray(offset, offset + length)));
        }

        await Promise.all(
            Array.from(sources).map(async source => {
                const img = await loadImage(source);
                this.textureCache.set(source, this.createTexture(img));
            })
        );
    }

    createTexture(img) {
        const gl = this.gl;
        const texture = gl.createTexture();

        gl.bindTexture(gl.TEXTURE_2D, texture);
        gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, img);

        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);

        return texture;
    }

    draw(numericBuffer, stringTable, drawStructuresCount) {
        const gl = this.gl;

        gl.viewport(0, 0, this.canvas.width, this.canvas.height);
        gl.disable(gl.SCISSOR_TEST);
        gl.clearColor(0, 0, 0, 0);
        gl.clear(gl.COLOR_BUFFER_BIT);

        gl.enable(gl.BLEND);
        gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);

        const decoder = new TextDecoder("utf-8");
        const pendingScrollbars = [];

        for(let i = 0; i < drawStructuresCount; i++) {
            const rowStart = i * PACKET_STRUCTURE.PACKET_SIZE;
            const type = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_TYPE];

            if(type === PACKET_STRUCTURE.PACKET_RECTANGLE_TYPE) {
                const x = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_X];
                const y = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_Y];

                const width = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_WIDTH];
                const height = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_HEIGHT];

                const isScrollableX = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_IS_SCROLLABLE_X];
                const isScrollableY = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_IS_SCROLLABLE_Y];

                const scrollableWidth = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_SCROLLABLE_WIDTH];
                const scrollableHeight = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_SCROLLABLE_HEIGHT];

                const scrollOffsetX = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_SCROLL_OFFSET_X];
                const scrollOffsetY = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_SCROLL_OFFSET_Y];

                const clipXStart = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_CLIP_X_START];
                const clipXEnd = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_CLIP_X_END];
                const clipYStart = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_CLIP_Y_START];
                const clipYEnd = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_CLIP_Y_END];

                const backgroundColor = unpackColor(numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BACKGROUND_COLOR]);
                const backgroundColorAlpha = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BACKGROUND_COLOR_ALPHA];

                const border = this.extractBorders(numericBuffer, rowStart);

                const opacity = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_OPACITY];

                this.applyScissor(clipXStart, clipXEnd, clipYStart, clipYEnd);
                this.drawRectangle(x, y, width, height, backgroundColor.r / 255, backgroundColor.g / 255, backgroundColor.b / 255, backgroundColorAlpha / 255, border, opacity);
            
                if(isScrollableX || isScrollableY) pendingScrollbars.push({ x, y, width, height, isScrollableX, isScrollableY, scrollableWidth, scrollableHeight, scrollOffsetX, scrollOffsetY, clipXStart, clipXEnd, clipYStart, clipYEnd });
            }
            
            else if(type === PACKET_STRUCTURE.PACKET_IMAGE_TYPE) {
                const x = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_X];
                const y = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_Y];

                const width = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_WIDTH];
                const height = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_HEIGHT];

                const clipXStart = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_CLIP_X_START];
                const clipXEnd = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_CLIP_X_END];
                const clipYStart = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_CLIP_Y_START];
                const clipYEnd = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_CLIP_Y_END];

                const offset = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_STRING_OFFSET];
                const length = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_STRING_LENGTH];

                const source = decoder.decode(stringTable.subarray(offset, offset + length));

                const texture = this.textureCache.get(source);
                if(!texture) continue;

                const backgroundColor = unpackColor(numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BACKGROUND_COLOR]);
                const backgroundColorAlpha = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BACKGROUND_COLOR_ALPHA];

                const border = this.extractBorders(numericBuffer, rowStart);

                const opacity = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_OPACITY];

                this.applyScissor(clipXStart, clipXEnd, clipYStart, clipYEnd);
                this.drawImage(x, y, width, height, texture, backgroundColor.r / 255, backgroundColor.g / 255, backgroundColor.b / 255, backgroundColorAlpha / 255, border, opacity);
            }
        }

        gl.disable(gl.SCISSOR_TEST);

        for(const pendingScrollbar of pendingScrollbars) {
            this.applyScissor(pendingScrollbar.clipXStart, pendingScrollbar.clipXEnd, pendingScrollbar.clipYStart, pendingScrollbar.clipYEnd);

            if(pendingScrollbar.isScrollableX) this.drawScrollbarX(pendingScrollbar.x, pendingScrollbar.y, pendingScrollbar.width, pendingScrollbar.height, pendingScrollbar.scrollableWidth, pendingScrollbar.scrollOffsetX);
            if(pendingScrollbar.isScrollableY) this.drawScrollbarY(pendingScrollbar.x, pendingScrollbar.y, pendingScrollbar.width, pendingScrollbar.height, pendingScrollbar.scrollableHeight, pendingScrollbar.scrollOffsetY);
        }
    }

    drawRectangle(x, y, width, height, backgroundColorR, backgroundColorG, backgroundColorB, backgroundColorAlpha, border, opacity) {
        const gl = this.gl;

        gl.useProgram(this.rectangleProgram);
        gl.uniform2f(this.rectangleResolutionLocation, this.canvas.width, this.canvas.height);
        gl.uniform2f(this.rectangleRectSizeLocation, width, height);
        gl.uniform4f(this.rectangleFillColorLocation, backgroundColorR, backgroundColorG, backgroundColorB, backgroundColorAlpha);
        gl.uniform1f(this.rectangleOpacityLocation, opacity);

        gl.uniform4f(this.rectangleBorderTopColorLocation, border.top.r, border.top.g, border.top.b, border.top.a);
        gl.uniform1f(this.rectangleBorderTopSizeLocation, border.top.size);
        gl.uniform1i(this.rectangleBorderTopTypeLocation, border.top.type);

        gl.uniform4f(this.rectangleBorderBottomColorLocation, border.bottom.r, border.bottom.g, border.bottom.b, border.bottom.a);
        gl.uniform1f(this.rectangleBorderBottomSizeLocation, border.bottom.size);
        gl.uniform1i(this.rectangleBorderBottomTypeLocation, border.bottom.type);

        gl.uniform4f(this.rectangleBorderLeftColorLocation, border.left.r, border.left.g, border.left.b, border.left.a);
        gl.uniform1f(this.rectangleBorderLeftSizeLocation, border.left.size);
        gl.uniform1i(this.rectangleBorderLeftTypeLocation, border.left.type);

        gl.uniform4f(this.rectangleBorderRightColorLocation, border.right.r, border.right.g, border.right.b, border.right.a);
        gl.uniform1f(this.rectangleBorderRightSizeLocation, border.right.size);
        gl.uniform1i(this.rectangleBorderRightTypeLocation, border.right.type);

        const positions = new Float32Array([
            x, y,
            x + width, y,
            x, y + height,
            x, y + height,
            x + width, y,
            x + width, y + height,
        ]);

        const localPositions = new Float32Array([
            0, 0,
            width, 0,
            0, height,
            0, height,
            width, 0,
            width, height,
        ]);

        gl.bindBuffer(gl.ARRAY_BUFFER, this.positionBuffer);
        gl.bufferData(gl.ARRAY_BUFFER, positions, gl.DYNAMIC_DRAW);
        gl.enableVertexAttribArray(this.rectanglePositionLocation);
        gl.vertexAttribPointer(this.rectanglePositionLocation, 2, gl.FLOAT, false, 0, 0);

        gl.bindBuffer(gl.ARRAY_BUFFER, this.localPositionBuffer);
        gl.bufferData(gl.ARRAY_BUFFER, localPositions, gl.DYNAMIC_DRAW);
        gl.enableVertexAttribArray(this.rectangleLocalPositionLocation);
        gl.vertexAttribPointer(this.rectangleLocalPositionLocation, 2, gl.FLOAT, false, 0, 0);

        gl.drawArrays(gl.TRIANGLES, 0, 6);
    }

    drawImage(x, y, width, height, texture, backgroundColorR, backgroundColorG, backgroundColorB, backgroundColorAlpha, border, opacity) {        
        const gl = this.gl;

        gl.useProgram(this.imageProgram);
        gl.uniform2f(this.imageResolutionLocation, this.canvas.width, this.canvas.height);
        gl.uniform2f(this.imageRectSizeLocation, width, height);
        gl.uniform4f(this.imageBackgroundColorLocation, backgroundColorR, backgroundColorG, backgroundColorB, backgroundColorAlpha);
        gl.uniform1f(this.imageOpacityLocation, opacity);

        gl.uniform4f(this.imageBorderTopColorLocation, border.top.r, border.top.g, border.top.b, border.top.a);
        gl.uniform1f(this.imageBorderTopSizeLocation, border.top.size);
        gl.uniform1i(this.imageBorderTopTypeLocation, border.top.type);

        gl.uniform4f(this.imageBorderBottomColorLocation, border.bottom.r, border.bottom.g, border.bottom.b, border.bottom.a);
        gl.uniform1f(this.imageBorderBottomSizeLocation, border.bottom.size);
        gl.uniform1i(this.imageBorderBottomTypeLocation, border.bottom.type);

        gl.uniform4f(this.imageBorderLeftColorLocation, border.left.r, border.left.g, border.left.b, border.left.a);
        gl.uniform1f(this.imageBorderLeftSizeLocation, border.left.size);
        gl.uniform1i(this.imageBorderLeftTypeLocation, border.left.type);

        gl.uniform4f(this.imageBorderRightColorLocation, border.right.r, border.right.g, border.right.b, border.right.a);
        gl.uniform1f(this.imageBorderRightSizeLocation, border.right.size);
        gl.uniform1i(this.imageBorderRightTypeLocation, border.right.type);

        const positions = new Float32Array([
            x, y,
            x + width, y,
            x, y + height,
            x, y + height,
            x + width, y,
            x + width, y + height,
        ]);

        const texCoords = new Float32Array([
            0, 0,
            1, 0,
            0, 1,
            0, 1,
            1, 0,
            1, 1,
        ]);

        const localPositions = new Float32Array([
            0, 0,
            width, 0,
            0, height,
            0, height,
            width, 0,
            width, height,
        ]);

        gl.bindBuffer(gl.ARRAY_BUFFER, this.positionBuffer);
        gl.bufferData(gl.ARRAY_BUFFER, positions, gl.DYNAMIC_DRAW);
        gl.enableVertexAttribArray(this.imagePositionLocation);
        gl.vertexAttribPointer(this.imagePositionLocation, 2, gl.FLOAT, false, 0, 0);

        gl.bindBuffer(gl.ARRAY_BUFFER, this.texCoordBuffer);
        gl.bufferData(gl.ARRAY_BUFFER, texCoords, gl.DYNAMIC_DRAW);
        gl.enableVertexAttribArray(this.imageTexCoordLocation);
        gl.vertexAttribPointer(this.imageTexCoordLocation, 2, gl.FLOAT, false, 0, 0);

        gl.bindBuffer(gl.ARRAY_BUFFER, this.localPositionBuffer);
        gl.bufferData(gl.ARRAY_BUFFER, localPositions, gl.DYNAMIC_DRAW);
        gl.enableVertexAttribArray(this.imageLocalPositionLocation);
        gl.vertexAttribPointer(this.imageLocalPositionLocation, 2, gl.FLOAT, false, 0, 0);

        gl.activeTexture(gl.TEXTURE0);
        gl.bindTexture(gl.TEXTURE_2D, texture);

        gl.drawArrays(gl.TRIANGLES, 0, 6);
    }

    drawScrollbarX(x, y, width, height, scrollableWidth, scrollOffsetX) {
        const trackHeight = 8;
        const trackY = y + height - trackHeight;

        this.drawScrollbarTrack(x, trackY, width, trackHeight, 0, 0, 0, 0.1);

        const thumbWidth = Math.max(20, width * (width / scrollableWidth));
        const maxOffset = Math.max(scrollableWidth - width, 1);
        const thumbX = x + (scrollOffsetX / maxOffset) * (width - thumbWidth);

        this.drawScrollbarTrack(thumbX, trackY, thumbWidth, trackHeight, 0, 0, 0, 0.4);
    }

    drawScrollbarY(x, y, width, height, scrollableHeight, scrollOffsetY) {
        const trackWidth = 8;
        const trackX = x + width - trackWidth;

        this.drawScrollbarTrack(trackX, y, trackWidth, height, 0, 0, 0, 0.1);

        const thumbHeight = Math.max(20, height * (height / scrollableHeight));
        const maxOffset = Math.max(scrollableHeight - height, 1);
        const thumbY = y + (scrollOffsetY / maxOffset) * (height - thumbHeight);

        this.drawScrollbarTrack(trackX, thumbY, trackWidth, thumbHeight, 0, 0, 0, 0.4);
    }

    drawScrollbarTrack(x, y, width, height, r, g, b, a) {
        const border = {
            top: { r: 0, g: 0, b: 0, a: 0, size: 0, type: 0 },
            bottom: { r: 0, g: 0, b: 0, a: 0, size: 0, type: 0 },
            left: { r: 0, g: 0, b: 0, a: 0, size: 0, type: 0 },
            right: { r: 0, g: 0, b: 0, a: 0, size: 0, type: 0 },
        };

        this.drawRectangle(x, y, width, height, r, g, b, a, border, 1);
    }

    extractBorders(numericBuffer, rowStart) {
        const build = (colorField, colorAlphaField, sizeField, typeField) => {
            const color = unpackColor(numericBuffer[rowStart + colorField]);
            const colorAlpha = numericBuffer[rowStart + colorAlphaField];

            return {
                r: color.r / 255,
                g: color.g / 255,
                b: color.b / 255,
                a: colorAlpha / 255,
                size: numericBuffer[rowStart + sizeField],
                type: numericBuffer[rowStart + typeField],
            };
        };

        return {
            top: build(PACKET_STRUCTURE.PACKET_BORDER_TOP_COLOR, PACKET_STRUCTURE.PACKET_BORDER_TOP_COLOR_ALPHA, PACKET_STRUCTURE.PACKET_BORDER_TOP_SIZE, PACKET_STRUCTURE.PACKET_BORDER_TOP_TYPE),
            bottom: build(PACKET_STRUCTURE.PACKET_BORDER_BOTTOM_COLOR, PACKET_STRUCTURE.PACKET_BORDER_BOTTOM_COLOR_ALPHA, PACKET_STRUCTURE.PACKET_BORDER_BOTTOM_SIZE, PACKET_STRUCTURE.PACKET_BORDER_BOTTOM_TYPE),
            left: build(PACKET_STRUCTURE.PACKET_BORDER_LEFT_COLOR, PACKET_STRUCTURE.PACKET_BORDER_LEFT_COLOR_ALPHA, PACKET_STRUCTURE.PACKET_BORDER_LEFT_SIZE, PACKET_STRUCTURE.PACKET_BORDER_LEFT_TYPE),
            right: build(PACKET_STRUCTURE.PACKET_BORDER_RIGHT_COLOR, PACKET_STRUCTURE.PACKET_BORDER_RIGHT_COLOR_ALPHA, PACKET_STRUCTURE.PACKET_BORDER_RIGHT_SIZE, PACKET_STRUCTURE.PACKET_BORDER_RIGHT_TYPE),
        };
    }

    applyScissor(clipXStart, clipXEnd, clipYStart, clipYEnd) {
        const gl = this.gl;

        const scissorWidth = Math.max(0, clipXEnd - clipXStart);
        const scissorHeight = Math.max(0, clipYEnd - clipYStart);

        // WebGL's scissor origin is bottom-left, but our clip coordinates are top-left, y-down.
        const scissorY = this.canvas.height - clipYEnd;

        gl.enable(gl.SCISSOR_TEST);
        gl.scissor(clipXStart, scissorY, scissorWidth, scissorHeight);
    }
}

function compileShader(gl, type, source) {
    const shader = gl.createShader(type);

    gl.shaderSource(shader, source);
    gl.compileShader(shader);

    if(!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
        const info = gl.getShaderInfoLog(shader);
        gl.deleteShader(shader);

        throw new Error(`Shader compile error: ${info}`);
    }

    return shader;
}

function createProgram(gl, vertexSource, fragmentSource) {
    const vertexShader = compileShader(gl, gl.VERTEX_SHADER, vertexSource);
    const fragmentShader = compileShader(gl, gl.FRAGMENT_SHADER, fragmentSource);

    const program = gl.createProgram();

    gl.attachShader(program, vertexShader);
    gl.attachShader(program, fragmentShader);

    gl.linkProgram(program);

    if(!gl.getProgramParameter(program, gl.LINK_STATUS)) {
        const info = gl.getProgramInfoLog(program);
        gl.deleteProgram(program);

        throw new Error(`Program link error: ${info}`);
    }

    return program;
}