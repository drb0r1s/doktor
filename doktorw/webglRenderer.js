const PACKET_SIZE = 16;

const PACKET_TYPE = 0;
const PACKET_X = 1;
const PACKET_Y = 2;
const PACKET_WIDTH_OR_FONT_SIZE = 3;
const PACKET_HEIGHT = 4;
const PACKET_R = 5;
const PACKET_G = 6;
const PACKET_B = 7;

const PACKET_RECTANGLE_TYPE = 0;
const PACKET_TEXT_TYPE = 1;
const PACKET_IMAGE_TYPE = 2;

const VERTEX_SHADER_SOURCE = `
    attribute vec2 a_position;
    uniform vec2 u_resolution;

    void main() {
        vec2 zeroToOne = a_position / u_resolution;
        vec2 zeroToTwo = zeroToOne * 2.0;
        vec2 clipSpace = zeroToTwo - 1.0;

        gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);
    }
`;

const FRAGMENT_SHADER_SOURCE = `
    precision mediump float;
    uniform vec4 u_color;

    void main() {
        gl_FragColor = u_color;
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

export class WebglRenderer {
    constructor(canvas) {
        const gl = canvas.getContext("webgl");

        if(!gl) throw new Error("WebGL not supported on this canvas");

        this.gl = gl;
        this.canvas = canvas;

        this.program = createProgram(gl, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE);

        this.positionLocation = gl.getAttribLocation(this.program, "a_position");
        this.resolutionLocation = gl.getUniformLocation(this.program, "u_resolution");
        this.colorLocation = gl.getUniformLocation(this.program, "u_color");

        this.positionBuffer = gl.createBuffer();
    }

    drawRectangles(numericBuffer, drawStructuresCount) {
        const gl = this.gl;

        gl.viewport(0, 0, this.canvas.width, this.canvas.height);
        gl.clearColor(0, 0, 0, 0);
        gl.clear(gl.COLOR_BUFFER_BIT);

        gl.useProgram(this.program);
        gl.uniform2f(this.resolutionLocation, this.canvas.width, this.canvas.height);

        gl.enable(gl.BLEND);
        gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);

        for(let i = 0; i < drawStructuresCount; i++) {
            const rowStart = i * PACKET_SIZE;
            const type = numericBuffer[rowStart + PACKET_TYPE];

            if(type !== PACKET_RECTANGLE_TYPE) continue;

            const x = numericBuffer[rowStart + PACKET_X];
            const y = numericBuffer[rowStart + PACKET_Y];

            const width = numericBuffer[rowStart + PACKET_WIDTH_OR_FONT_SIZE];
            const height = numericBuffer[rowStart + PACKET_HEIGHT];

            const r = numericBuffer[rowStart + PACKET_R] / 255;
            const g = numericBuffer[rowStart + PACKET_G] / 255;
            const b = numericBuffer[rowStart + PACKET_B] / 255;

            this.drawTriangle(x, y, width, height, r, g, b, 1.0);
        }
    }

    drawTriangle(x, y, width, height, r, g, b, a) {
        const gl = this.gl;

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

        gl.enableVertexAttribArray(this.positionLocation);
        gl.vertexAttribPointer(this.positionLocation, 2, gl.FLOAT, false, 0, 0);

        gl.uniform4f(this.colorLocation, r, g, b, a);

        gl.drawArrays(gl.TRIANGLES, 0, 6);
    }
}