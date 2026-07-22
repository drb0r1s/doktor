const PACKET_SIZE = 16;

const PACKET_TYPE = 0;
const PACKET_X = 1;
const PACKET_Y = 2;
const PACKET_WIDTH_OR_FONT_SIZE = 3;
const PACKET_HEIGHT = 4;
const PACKET_R = 5;
const PACKET_G = 6;
const PACKET_B = 7;
const PACKET_STRING_OFFSET = 8;
const PACKET_STRING_LENGTH = 9;

const PACKET_RECTANGLE_TYPE = 0;
const PACKET_TEXT_TYPE = 1;
const PACKET_IMAGE_TYPE = 2;

const RECTANGLE_VERTEX_SHADER_SOURCE = `
    attribute vec2 a_position;
    uniform vec2 u_resolution;

    void main() {
        vec2 zeroToOne = a_position / u_resolution;
        vec2 zeroToTwo = zeroToOne * 2.0;
        vec2 clipSpace = zeroToTwo - 1.0;

        gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);
    }
`;

const RECTANGLE_FRAGMENT_SHADER_SOURCE = `
    precision mediump float;
    uniform vec4 u_color;

    void main() {
        gl_FragColor = u_color;
    }
`;

const IMAGE_VERTEX_SHADER_SOURCE = `
    attribute vec2 a_position;
    attribute vec2 a_texCoord;
    uniform vec2 u_resolution;
    varying vec2 v_texCoord;

    void main() {
        vec2 zeroToOne = a_position / u_resolution;
        vec2 zeroToTwo = zeroToOne * 2.0;
        vec2 clipSpace = zeroToTwo - 1.0;

        gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);
        v_texCoord = a_texCoord;
    }
`;

const IMAGE_FRAGMENT_SHADER_SOURCE = `
    precision mediump float;
    uniform sampler2D u_image;
    varying vec2 v_texCoord;

    void main() {
        gl_FragColor = texture2D(u_image, v_texCoord);
    }
`;

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
        this.rectangleResolutionLocation = gl.getUniformLocation(this.rectangleProgram, "u_resolution");
        this.rectangleColorLocation = gl.getUniformLocation(this.rectangleProgram, "u_color");

        // Image Program
        this.imageProgram = createProgram(gl, IMAGE_VERTEX_SHADER_SOURCE, IMAGE_FRAGMENT_SHADER_SOURCE);
        this.imagePositionLocation = gl.getAttribLocation(this.imageProgram, "a_position");
        this.imageTexCoordLocation = gl.getAttribLocation(this.imageProgram, "a_texCoord");
        this.imageResolutionLocation = gl.getUniformLocation(this.imageProgram, "u_resolution");

        // Shared Position Buffer
        this.positionBuffer = gl.createBuffer();
        this.texCoordBuffer = gl.createBuffer();

        this.textureCache = new Map(); // Image source -> WebGL texture
    }

    // Called once before drawing, so that textures are ready before drawing Image blocks.
    async preloadTextures(numericBuffer, stringTable, drawStructuresCount) {
        const decoder = new TextDecoder("utf-8");
        const sources = new Set();

        for(let i = 0; i < drawStructuresCount; i++) {
            const rowStart = i * PACKET_SIZE;
            if(numericBuffer[rowStart + PACKET_TYPE] !== PACKET_IMAGE_TYPE) continue;

            const offset = numericBuffer[rowStart + PACKET_STRING_OFFSET];
            const length = numericBuffer[rowStart + PACKET_STRING_LENGTH];

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
        gl.clearColor(0, 0, 0, 0);
        gl.clear(gl.COLOR_BUFFER_BIT);

        gl.enable(gl.BLEND);
        gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);

        const decoder = new TextDecoder("utf-8");

        for(let i = 0; i < drawStructuresCount; i++) {
            const rowStart = i * PACKET_SIZE;
            const type = numericBuffer[rowStart + PACKET_TYPE];

            if(type === PACKET_RECTANGLE_TYPE) {
                const x = numericBuffer[rowStart + PACKET_X];
                const y = numericBuffer[rowStart + PACKET_Y];

                const width = numericBuffer[rowStart + PACKET_WIDTH_OR_FONT_SIZE];
                const height = numericBuffer[rowStart + PACKET_HEIGHT];

                const r = numericBuffer[rowStart + PACKET_R] / 255;
                const g = numericBuffer[rowStart + PACKET_G] / 255;
                const b = numericBuffer[rowStart + PACKET_B] / 255;

                this.drawRectangle(x, y, width, height, r, g, b, 1.0);
            }
            
            else if(type === PACKET_IMAGE_TYPE) {
                const x = numericBuffer[rowStart + PACKET_X];
                const y = numericBuffer[rowStart + PACKET_Y];

                const width = numericBuffer[rowStart + PACKET_WIDTH_OR_FONT_SIZE];
                const height = numericBuffer[rowStart + PACKET_HEIGHT];

                const offset = numericBuffer[rowStart + PACKET_STRING_OFFSET];
                const length = numericBuffer[rowStart + PACKET_STRING_LENGTH];

                const source = decoder.decode(stringTable.subarray(offset, offset + length));

                const texture = this.textureCache.get(source);
                if(!texture) continue; // An error happened while trying to preload the texture, do not render the Image.

                this.drawImage(x, y, width, height, texture);
            }
        }
    }

    drawRectangle(x, y, width, height, r, g, b, a) {
        const gl = this.gl;

        gl.useProgram(this.rectangleProgram);
        gl.uniform2f(this.rectangleResolutionLocation, this.canvas.width, this.canvas.height);

        const positions = new Float32Array([
            x, y,
            x + width, y,
            x, y + height,
            x, y + height,
            x + width, y,
            x + width, y + height,
        ]);

        gl.bindBuffer(gl.ARRAY_BUFFER, this.positionBuffer);
        gl.bufferData(gl.ARRAY_BUFFER, positions, gl.DYNAMIC_DRAW);

        gl.enableVertexAttribArray(this.rectanglePositionLocation);
        gl.vertexAttribPointer(this.rectanglePositionLocation, 2, gl.FLOAT, false, 0, 0);

        gl.uniform4f(this.rectangleColorLocation, r, g, b, a);

        gl.drawArrays(gl.TRIANGLES, 0, 6);
    }

    drawImage(x, y, width, height, texture) {
        const gl = this.gl;

        gl.useProgram(this.imageProgram);
        gl.uniform2f(this.imageResolutionLocation, this.canvas.width, this.canvas.height);

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

        gl.bindBuffer(gl.ARRAY_BUFFER, this.positionBuffer);
        gl.bufferData(gl.ARRAY_BUFFER, positions, gl.DYNAMIC_DRAW);
        gl.enableVertexAttribArray(this.imagePositionLocation);
        gl.vertexAttribPointer(this.imagePositionLocation, 2, gl.FLOAT, false, 0, 0);

        gl.bindBuffer(gl.ARRAY_BUFFER, this.texCoordBuffer);
        gl.bufferData(gl.ARRAY_BUFFER, texCoords, gl.DYNAMIC_DRAW);
        gl.enableVertexAttribArray(this.imageTexCoordLocation);
        gl.vertexAttribPointer(this.imageTexCoordLocation, 2, gl.FLOAT, false, 0, 0);

        gl.activeTexture(gl.TEXTURE0);
        gl.bindTexture(gl.TEXTURE_2D, texture);

        gl.drawArrays(gl.TRIANGLES, 0, 6);
    }
}