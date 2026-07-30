import { unpackColor } from "../../functions/unpackColor.js";
import { RECTANGLE_VERTEX_SHADER_SOURCE, RECTANGLE_FRAGMENT_SHADER_SOURCE } from "./shaders/rectangle.js";
import { IMAGE_VERTEX_SHADER_SOURCE, IMAGE_FRAGMENT_SHADER_SOURCE } from "./shaders/image.js";
import { PACKET_STRUCTURE } from "../../data/packetStructure.js";

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

function loadImage(source) {
    return new Promise((resolve, reject) => {
        const img = new window.Image();

        img.onload = () => resolve(img);
        img.onerror = () => reject(new Error(`Failed to load image: ${source}`));

        img.src = source;
    });
}

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
        this.rectangleBorderColorLocation = gl.getUniformLocation(this.rectangleProgram, "u_borderColor");
        this.rectangleBorderSizeLocation = gl.getUniformLocation(this.rectangleProgram, "u_borderSize");
        this.rectangleBorderTypeLocation = gl.getUniformLocation(this.rectangleProgram, "u_borderType");
        this.rectangleRectSizeLocation = gl.getUniformLocation(this.rectangleProgram, "u_rectSize");
        this.rectangleOpacityLocation = gl.getUniformLocation(this.rectangleProgram, "u_opacity");

        // Image Program
        this.imageProgram = createProgram(gl, IMAGE_VERTEX_SHADER_SOURCE, IMAGE_FRAGMENT_SHADER_SOURCE);
        this.imagePositionLocation = gl.getAttribLocation(this.imageProgram, "a_position");
        this.imageTexCoordLocation = gl.getAttribLocation(this.imageProgram, "a_texCoord");
        this.imageResolutionLocation = gl.getUniformLocation(this.imageProgram, "u_resolution");
        this.imageOpacityLocation = gl.getUniformLocation(this.imageProgram, "u_opacity");
        this.imageLocalPositionLocation = gl.getAttribLocation(this.imageProgram, "a_localPosition");
        this.imageBackgroundColorLocation = gl.getUniformLocation(this.imageProgram, "u_backgroundColor");
        this.imageBorderColorLocation = gl.getUniformLocation(this.imageProgram, "u_borderColor");
        this.imageBorderSizeLocation = gl.getUniformLocation(this.imageProgram, "u_borderSize");
        this.imageBorderTypeLocation = gl.getUniformLocation(this.imageProgram, "u_borderType");
        this.imageRectSizeLocation = gl.getUniformLocation(this.imageProgram, "u_rectSize");

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

        for(let i = 0; i < drawStructuresCount; i++) {
            const rowStart = i * PACKET_STRUCTURE.PACKET_SIZE;
            const type = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_TYPE];

            if(type === PACKET_STRUCTURE.PACKET_RECTANGLE_TYPE) {
                const x = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_X];
                const y = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_Y];

                const width = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_WIDTH];
                const height = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_HEIGHT];

                const clipXStart = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_CLIP_X_START];
                const clipXEnd = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_CLIP_X_END];
                const clipYStart = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_CLIP_Y_START];
                const clipYEnd = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_CLIP_Y_END];

                const backgroundColor = unpackColor(numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BACKGROUND_COLOR]);
                const backgroundColorAlpha = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BACKGROUND_COLOR_ALPHA];

                const borderColor = unpackColor(numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BORDER_COLOR]);
                const borderColorAlpha = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BORDER_COLOR_ALPHA];

                const borderSize = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BORDER_SIZE];
                const borderType = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BORDER_TYPE];

                const opacity = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_OPACITY];

                this.applyScissor(clipXStart, clipXEnd, clipYStart, clipYEnd);
                this.drawRectangle(x, y, width, height, backgroundColor.r / 255, backgroundColor.g / 255, backgroundColor.b / 255, backgroundColorAlpha / 255, borderColor.r / 255, borderColor.g / 255, borderColor.b / 255, borderColorAlpha / 255, borderSize, borderType, opacity);
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

                const borderColor = unpackColor(numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BORDER_COLOR]);
                const borderColorAlpha = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BORDER_COLOR_ALPHA];

                const borderSize = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BORDER_SIZE];
                const borderType = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_BORDER_TYPE];

                const opacity = numericBuffer[rowStart + PACKET_STRUCTURE.PACKET_OPACITY];

                this.applyScissor(clipXStart, clipXEnd, clipYStart, clipYEnd);
                this.drawImage(x, y, width, height, texture, backgroundColor.r / 255, backgroundColor.g / 255, backgroundColor.b / 255, backgroundColorAlpha / 255, borderColor.r / 255, borderColor.g / 255, borderColor.b / 255, borderColorAlpha / 255, borderSize, borderType, opacity);
            }
        }
    }

    drawRectangle(x, y, width, height, backgroundColorR, backgroundColorG, backgroundColorB, backgroundColorAlpha, borderColorR, borderColorG, borderColorB, borderColorAlpha, borderSize, borderType, opacity) {
        const gl = this.gl;

        gl.useProgram(this.rectangleProgram);
        gl.uniform2f(this.rectangleResolutionLocation, this.canvas.width, this.canvas.height);
        gl.uniform2f(this.rectangleRectSizeLocation, width, height);
        gl.uniform4f(this.rectangleFillColorLocation, backgroundColorR, backgroundColorG, backgroundColorB, backgroundColorAlpha);
        gl.uniform4f(this.rectangleBorderColorLocation, borderColorR, borderColorG, borderColorB, borderColorAlpha);
        gl.uniform1f(this.rectangleBorderSizeLocation, borderSize);
        gl.uniform1i(this.rectangleBorderTypeLocation, borderType);
        gl.uniform1f(this.rectangleOpacityLocation, opacity);

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

    drawImage(x, y, width, height, texture, backgroundColorR, backgroundColorG, backgroundColorB, backgroundColorAlpha, borderColorR, borderColorG, borderColorB, borderColorAlpha, borderSize, borderType, opacity) {
        const gl = this.gl;

        gl.useProgram(this.imageProgram);
        gl.uniform2f(this.imageResolutionLocation, this.canvas.width, this.canvas.height);
        gl.uniform2f(this.imageRectSizeLocation, width, height);
        gl.uniform4f(this.imageBackgroundColorLocation, backgroundColorR, backgroundColorG, backgroundColorB, backgroundColorAlpha);
        gl.uniform4f(this.imageBorderColorLocation, borderColorR, borderColorG, borderColorB, borderColorAlpha);
        gl.uniform1f(this.imageBorderSizeLocation, borderSize);
        gl.uniform1i(this.imageBorderTypeLocation, borderType);
        gl.uniform1f(this.imageOpacityLocation, opacity);

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