/// <reference lib="deno.window" />

import * as esbuild from "https://deno.land/x/esbuild@v0.19.4/mod.js";
import { denoPlugins } from "https://deno.land/x/esbuild_deno_loader@0.8.2/mod.ts";
import { parse } from "$std/flags/mod.ts";
import { join } from "$std/path/mod.ts";
import { parse as parseDOM } from "npm:node-html-parser";
import { copy, exists } from "$std/fs/mod.ts";

const args = parse(Deno.args, {
    boolean: ["dev"],
});
const is_dev = args.dev;

const mode = args._[0];

const indexFile = await Deno.readTextFile("index.html");

const documentRoot = parseDOM(indexFile);

if (documentRoot === null) {
    throw new Error(`entry document was not exists.`);
}

const scriptElems = documentRoot.querySelectorAll(`script[type="module"][src]`);
for (const s of scriptElems) {
    s.remove();
}
const scriptSources = scriptElems.map(s => s.getAttribute("src")!);

const configPath = join(Deno.cwd(), "deno.json");

const commonOptions: esbuild.BuildOptions = {
    plugins: [
        ...denoPlugins({ configPath }),
    ],
    entryPoints: scriptSources,
    outdir: `./dist`,
    bundle: true,
    format: "esm",
    platform: "browser",
};

const scriptElem = parseDOM(`<script type="module" src="/index.js"></script>`);

const document = documentRoot.getElementsByTagName("html")[0];

document.appendChild(scriptElem);

if (await exists("public")) {
    await copy("public", "dist", { overwrite: true });
} else {
    await Deno.mkdir("dist", { recursive: true });
}
const outIndexFile = await Deno.create("dist/index.html");
await outIndexFile.write(new TextEncoder().encode(documentRoot.toString()));

let option: esbuild.BuildOptions;
if (is_dev) {
    option = commonOptions;
} else {
    option = {
        ...commonOptions, dropLabels: ["DEV"],
        minifySyntax: true,
    };
}

const context = await esbuild.context(option);

if (mode === "serve") {
    const { host, port } = await context.serve({ port: 1420, servedir: "dist", });

    const origin = host === "0.0.0.0" ? "localhost" : host;

    console.log(`Serving on http://${origin}:${port}`);
    const watcher = Deno.watchFs(["src"]);
    console.log("Watching...");
    for await (const e of watcher) {
        console.log(`File Update: (${e.kind}) ${e.paths}.`);
        await context.rebuild();
    }
}
if (mode === "build") {
    const result = await context.rebuild();
    console.log(result);
}

esbuild.stop();