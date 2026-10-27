import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { defineConfig } from "tsdown";

const pkgPath = resolve(process.cwd(), "package.json");
const pkg = JSON.parse(readFileSync(pkgPath, "utf-8"));
const allDependencies = Object.keys(pkg.dependencies || {});

export default defineConfig({
	entry: ["src/main.ts"],
	outDir: "./packs/BP/scripts",
	format: ["esm"],
	sourcemap: false,
	deps: {
		neverBundle: ["@minecraft/server", "@minecraft/server-ui"],
		alwaysBundle: allDependencies,
	},
	outputOptions: {
		comments: false,
		entryFileNames: "[name].js",
	},
});
