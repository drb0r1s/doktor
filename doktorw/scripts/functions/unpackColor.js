export function unpackColor(packedColor) {
    const value = packedColor >>> 0; // unsigned

    const r = (value >> 16) & 0xFF;
    const g = (value >> 8) & 0xFF;
    const b = value & 0xFF;

    return { r, g, b };
}