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
    uniform vec4 u_borderColor;
    uniform float u_borderSize;
    uniform int u_borderType;
    uniform vec2 u_rectSize;
    uniform float u_opacity;

    varying vec2 v_localPosition;

    void main() {
        float distanceToLeft = v_localPosition.x;
        float distanceToRight = u_rectSize.x - v_localPosition.x;
        float distanceToTop = v_localPosition.y;
        float distanceToBottom = u_rectSize.y - v_localPosition.y;

        float distanceToEdge = min(min(distanceToLeft, distanceToRight), min(distanceToTop, distanceToBottom));

        bool insideBorderBand = u_borderSize > 0.0 && distanceToEdge < u_borderSize;

        vec4 resultColor;

        if(insideBorderBand && u_borderType == 1) {
            resultColor = u_borderColor;
        }
        
        else if(insideBorderBand && (u_borderType == 2 || u_borderType == 3)) {
            float dashLength = u_borderType == 2 ? 12.0 : 4.0;
            float gapLength = u_borderType == 2 ? 8.0 : 6.0;

            float period = dashLength + gapLength;

            float alongEdge;
            
            // Top edge
            if(distanceToTop <= distanceToBottom && distanceToTop <= distanceToLeft && distanceToTop <= distanceToRight) {
                alongEdge = v_localPosition.x;
            }
            
            // Bottom edge
            else if(distanceToBottom <= distanceToLeft && distanceToBottom <= distanceToRight) {
                alongEdge = v_localPosition.x;
            }
            
            // Left edge
            else if(distanceToLeft <= distanceToRight) {
                alongEdge = v_localPosition.y;
            }
            
            // Right edge
            else {
                alongEdge = v_localPosition.y;
            }

            float positionInPeriod = mod(alongEdge, period);

            if(positionInPeriod < dashLength) {
                resultColor = u_borderColor;
            }
                
            else {
                resultColor = u_fillColor;
            }
        }
            
        else {
            resultColor = u_fillColor;
        }

        resultColor.a *= u_opacity;
        gl_FragColor = resultColor;
    }
`;