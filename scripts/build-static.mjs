import { mkdirSync, copyFileSync, readFileSync, writeFileSync, existsSync, rmSync } from "node:fs";
import { dirname, join } from "node:path";

const root = process.cwd();
const dist = join(root, "dist");
const assets = join(dist, "assets");
const server = join(dist, "server");
rmSync(dist, { recursive: true, force: true });
mkdirSync(assets, { recursive: true });
mkdirSync(server, { recursive: true });
mkdirSync(join(dist, ".openai"), { recursive: true });

copyFileSync(join(root, "frontend", "index.html"), join(dist, "index.html"));
copyFileSync(join(root, "frontend", "src", "styles.css"), join(assets, "styles.css"));
copyFileSync(join(root, "frontend", "src", "app.js"), join(assets, "app.js"));
copyFileSync(join(root, ".openai", "hosting.json"), join(dist, ".openai", "hosting.json"));

const html = readFileSync(join(dist, "index.html"), "utf8");
const css = readFileSync(join(assets, "styles.css"), "utf8");
const js = readFileSync(join(assets, "app.js"), "utf8");
const worker = `const files = new Map([["/", { body: ${JSON.stringify(html)}, type: "text/html; charset=utf-8" }],["/index.html", { body: ${JSON.stringify(html)}, type: "text/html; charset=utf-8" }],["/assets/styles.css", { body: ${JSON.stringify(css)}, type: "text/css; charset=utf-8" }],["/assets/app.js", { body: ${JSON.stringify(js)}, type: "text/javascript; charset=utf-8" }]]);\nexport default { async fetch(request) { const url = new URL(request.url); const file = files.get(url.pathname) || files.get("/"); return new Response(file.body, { headers: { "content-type": file.type, "cache-control": url.pathname === "/" ? "no-store" : "public, max-age=31536000, immutable" } }); } };\n`;
writeFileSync(join(server, "index.js"), worker);

if (process.argv.includes("--check") && !existsSync(join(server, "index.js"))) {
  throw new Error("Build output missing server entrypoint");
}
console.log("ClashAI static deployment built in dist/");
