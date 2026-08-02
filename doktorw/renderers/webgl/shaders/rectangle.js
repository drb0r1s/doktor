export const RECTANGLE_VERTEX_SHADER_SOURCE = `
    attribute vec2 a_position;
    attribute vec2 a_localPosition;
    uniform vec2 u_resolution;
    varying vec2 v_localPosition;

    void main() {
        vec2 zeroToOne = a_position / u_resolution;
        vec2 zeroToTwo = zeroToOne * 2.0;
        vec2 clipSpace = zeroToTwo - 1.0;

        gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);
        v_localPosition = a_localPosition;
    }
`;

export const RECTANGLE_FRAGMENT_SHADER_SOURCE = `
    precision mediump float;

    uniform vec4 u_fillColor;

    uniform vec4 u_borderTopColor;
    uniform float u_borderTopSize;
    uniform int u_borderTopType;

    uniform vec4 u_borderBottomColor;
    uniform float u_borderBottomSize;
    uniform int u_borderBottomType;

    uniform vec4 u_borderLeftColor;
    uniform float u_borderLeftSize;
    uniform int u_borderLeftType;

    uniform vec4 u_borderRightColor;
    uniform float u_borderRightSize;
    uniform int u_borderRightType;

    uniform vec2 u_rectSize;
    uniform float u_opacity;

    varying vec2 v_localPosition;

    vec4 dashedColor(float alongEdge, int borderType, vec4 borderColor, vec4 baseColor) {
        float dashLength = borderType == 2 ? 12.0 : 4.0;
        float gapLength = borderType == 2 ? 8.0 : 6.0;
        float period = dashLength + gapLength;
        float positionInPeriod = mod(alongEdge, period);

        return positionInPeriod < dashLength ? borderColor : baseColor;
    }

    void main() {
        float distanceToLeft = v_localPosition.x;
        float distanceToRight = u_rectSize.x - v_localPosition.x;
        float distanceToTop = v_localPosition.y;
        float distanceToBottom = u_rectSize.y - v_localPosition.y;

        bool inTopBand = u_borderTopSize > 0.0 && distanceToTop < u_borderTopSize;
        bool inBottomBand = u_borderBottomSize > 0.0 && distanceToBottom < u_borderBottomSize;
        bool inLeftBand = u_borderLeftSize > 0.0 && distanceToLeft < u_borderLeftSize;
        bool inRightBand = u_borderRightSize > 0.0 && distanceToRight < u_borderRightSize;

        vec4 resultColor = u_fillColor;

        if(inTopBand && distanceToTop <= distanceToLeft && distanceToTop <= distanceToRight) {
            resultColor = u_borderTopType == 1 ? u_borderTopColor : (u_borderTopType == 2 || u_borderTopType == 3 ? dashedColor(v_localPosition.x, u_borderTopType, u_borderTopColor, u_fillColor) : u_fillColor);
        }

        else if(inBottomBand && distanceToBottom <= distanceToLeft && distanceToBottom <= distanceToRight) {
            resultColor = u_borderBottomType == 1 ? u_borderBottomColor : (u_borderBottomType == 2 || u_borderBottomType == 3 ? dashedColor(v_localPosition.x, u_borderBottomType, u_borderBottomColor, u_fillColor) : u_fillColor);
        }

        else if(inLeftBand) {
            resultColor = u_borderLeftType == 1 ? u_borderLeftColor : (u_borderLeftType == 2 || u_borderLeftType == 3 ? dashedColor(v_localPosition.y, u_borderLeftType, u_borderLeftColor, u_fillColor) : u_fillColor);
        }

        else if(inRightBand) {
            resultColor = u_borderRightType == 1 ? u_borderRightColor : (u_borderRightType == 2 || u_borderRightType == 3 ? dashedColor(v_localPosition.y, u_borderRightType, u_borderRightColor, u_fillColor) : u_fillColor);
        }

        resultColor.a *= u_opacity;
        gl_FragColor = resultColor;
    }
`;