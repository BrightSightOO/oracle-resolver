// @ts-check

import fs from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { rootNodeFromAnchor } from "@codama/nodes-from-anchor";
import * as kJs from "@codama/renderers-js-umi";
import * as kRust from "@codama/renderers-rust";
import * as k from "codama";
import { bold } from "colorette";
import { ESLint } from "eslint";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const rootDir = path.dirname(__dirname);
const idlDir = path.join(rootDir, "idls");
const clientDir = path.join(rootDir, "clients");

const idlJson = await fs.readFile(path.join(idlDir, "oracle_resolver.json"), "utf8");
const idl = JSON.parse(idlJson);

const start = Date.now();

console.log("generating clients...");

const codama = k.createFromRoot(rootNodeFromAnchor(idl));

// Update accounts.
codama.update(k.updateAccountsVisitor({}));

// Set default values for instruction accounts.
codama.update(k.setInstructionAccountDefaultValuesVisitor([]));

// Update instructions.
codama.update(k.updateInstructionsVisitor({}));

/** @param {string} name */
const accountType = (name) => ({
  field: "accountType",
  value: k.enumValueNode("AccountType", name),
});

// Set account discriminators.
codama.update(
  k.setAccountDiscriminatorFromFieldVisitor({
    ResolverV1: accountType("ResolverV1"),
  }),
);

// Render Rust.
{
  const crateDir = path.join(clientDir, "rust");
  const rustDir = path.join(crateDir, "src", "generated");

  console.log(`writing rust client to ${bold(path.relative(rootDir, rustDir))}...`);

  codama.accept(
    kRust.renderVisitor(rustDir, {
      crateFolder: crateDir,
      formatCode: true,
      toolchain: "+nightly",
      linkOverrides: {},
    }),
  );
}

// Render JavaScript.
{
  const jsDir = path.join(clientDir, "js", "src", "generated");

  console.log(`writing js client to ${bold(path.relative(rootDir, jsDir))}...`);

  await codama.accept(
    kJs.renderVisitor(jsDir, {
      formatCode: true,
      linkOverrides: {},
    }),
  );

  console.log("cleaning up generated js client code...");

  const eslint = new ESLint({
    cache: true,
    cacheLocation: path.join(rootDir, "node_modules", ".cache", "eslint-codama"),
    cacheStrategy: "content",
    fix: true,
  });
  const lintResults = await eslint.lintFiles(jsDir);

  await ESLint.outputFixes(lintResults);

  const eslintFormatter = await eslint.loadFormatter();
  const lintOutput = await eslintFormatter.format(lintResults);

  if (lintOutput) {
    console.error(lintOutput);
  }
}

console.log(`done in ${bold(`${Date.now() - start}ms`)}`);
