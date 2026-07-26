export const IMAGE_VERTEX_SHADER_SOURCE = `
    attribute vec2 a_position;
    attribute vec2 a_texCoord;
    attribute vec2 a_localPosition;
    uniform vec2 u_resolution;
    varying vec2 v_texCoord;
    varying vec2 v_localPosition;

    void main() {
        vec2 zeroToOne = a_position / u_resolution;
        vec2 zeroToTwo = zeroToOne * 2.0;
        vec2 clipSpace = zeroToTwo - 1.0;

        gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);
        v_texCoord = a_texCoord;
        v_localPosition = a_localPosition;
    }
`;

export const IMAGE_FRAGMENT_SHADER_SOURCE = `
    precision mediump float;

    uniform sampler2D u_image;
    uniform vec4 u_backgroundColor;
    uniform vec4 u_borderColor;
    uniform float u_borderSize;
    uniform int u_borderType;
    uniform vec2 u_rectSize;
    uniform float u_opacity;

    varying vec2 v_texCoord;
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

            if(distanceToTop <= distanceToBottom && distanceToTop <= distanceToLeft && distanceToTop <= distanceToRight) {
                alongEdge = v_localPosition.x;
            }
                
            else if (distanceToBottom <= distanceToLeft && distanceToBottom <= distanceToRight) {
                alongEdge = v_localPosition.x;
            }
                
            else if (distanceToLeft <= distanceToRight) {
                alongEdge = v_localPosition.y;
            }
                
            else {
                alongEdge = v_localPosition.y;
            }

            float positionInPeriod = mod(alongEdge, period);
            
            vec4 texColor = texture2D(u_image, v_texCoord);
            vec4 blended = mix(u_backgroundColor, texColor, texColor.a);
            
            resultColor = positionInPeriod < dashLength ? u_borderColor : blended;
        }
            
        else {
            vec4 texColor = texture2D(u_image, v_texCoord);
            resultColor = mix(u_backgroundColor, texColor, texColor.a);
        }

        resultColor.a *= u_opacity;
        gl_FragColor = resultColor;
    }
`;