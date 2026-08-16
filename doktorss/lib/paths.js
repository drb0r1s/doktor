const path = require("path");

const ROOT = path.resolve(__dirname, "..", "..");

const DOKTORC = path.join(ROOT, "doktorc");
const DOKTORR = path.join(ROOT, "doktorr");

const DOKTORC_OUT = path.join(DOKTORC, "out");
const DOKTORC_DEBUG = path.join(DOKTORC, "target", "debug");

const EXE_NAME = process.platform === "win32" ? "doktorc.exe" : "doktorc";

const DOKTORC_DEBUG_EXE = path.join(DOKTORC_DEBUG, EXE_NAME);
const DOKTORC_EXE = path.join(DOKTORC_OUT, EXE_NAME);

const PORT = 9999;

module.exports = {
    ROOT,
    DOKTORC,
    DOKTORR,
    DOKTORC_OUT,
    DOKTORC_DEBUG,
    EXE_NAME,
    DOKTORC_DEBUG_EXE,
    DOKTORC_EXE,
    PORT,
};