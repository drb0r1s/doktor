const http = require("http");
const fs = require("fs");
const path = require("path");

const { ROOT } = require("./paths");

const MIME_TYPES = {
    ".html": "text/html",
    ".js": "text/javascript",
    ".mjs": "text/javascript",
    ".wasm": "application/wasm",
    ".json": "application/json",
    ".css": "text/css",
    ".doktorb": "application/octet-stream",
    ".png": "image/png",
    ".jpg": "image/jpeg",
    ".jpeg": "image/jpeg",
    ".svg": "image/svg+xml",
};

// The code to be injected inside the index.html of a DOKTOR project.
const LIVE_RELOAD = `
    <!-- This script tag has been injected by the DOKTOR Server. -->

    <script>
        (function () {
            let socket = new WebSocket("ws://" + location.hostname + ":${"__PORT__"}");

            socket.addEventListener("message", e => {
                if(event.data === "reload") location.reload();
            });
        })();
    </script>
`;

function createStaticServer(rootDirectory, port) {
    const injection = LIVE_RELOAD.replace("__PORT__", String(port));

    return http.createServer((req, res) => {
        const requestPath = decodeURIComponent(req.url.split("?")[0]);
        const resolvedPath = path.join(rootDirectory, requestPath === "/" ? "/index.html" : requestPath);

        // Prevent escaping rootDirectory via "../" traversal.
        if(!resolvedPath.startsWith(rootDirectory)) {
            res.writeHead(403);
            res.end("Forbidden");

            return;
        }

        fs.readFile(resolvedPath, (error, data) => {
            if(error) {
                const pkgResolvedPath = path.join(ROOT, requestPath);
                
                fs.readFile(pkgResolvedPath, (pkgError, pkgData) => {
                    if(pkgError) {
                        res.writeHead(404);
                        res.end("Not found");

                        return;
                    }

                    const extension = path.extname(pkgResolvedPath);
                    const contentType = MIME_TYPES[extension] ?? "application/octet-stream";

                    res.setHeader("Content-Type", contentType);
                    res.setHeader("Cache-Control", "no-store");

                    res.end(pkgData);
                });

                return;
            }

            const extension = path.extname(resolvedPath);
            const contentType = MIME_TYPES[extension] ?? "application/octet-stream";

            res.setHeader("Content-Type", contentType);
            res.setHeader("Cache-Control", "no-store");

            if(extension === ".html") {
                const html = data.toString("utf-8").replace("</body>", `${injection}</body>`);
                res.end(html);
            }
            
            else res.end(data);
        });
    });
}

module.exports = { createStaticServer };