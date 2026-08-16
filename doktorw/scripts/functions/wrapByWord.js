export function wrapByWord(context, content, maxWidth) {
    const words = content.split(" ");
    const lines = [];

    let currentLine = "";

    for(const word of words) {
        const candidate = currentLine.length === 0 ? word : `${currentLine} ${word}`;

        if(context.measureText(candidate).width > maxWidth && currentLine.length > 0) {
            lines.push(currentLine);
            currentLine = word;
        }
        
        else currentLine = candidate;
    }

    if(currentLine.length > 0) lines.push(currentLine);
    if(lines.length === 0) lines.push("");

    return lines;
}