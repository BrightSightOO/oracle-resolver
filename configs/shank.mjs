// @ts-check

import path from "node:path";
import { fileURLToPath } from "node:url";

import { generateIdl } from "@metaplex-foundation/shank-js";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const rootDir = path.dirname(__dirname);

const idlDir = path.join(rootDir, "idls");
const programDir = path.join(rootDir, "programs");
const binaryInstallDir = path.join(rootDir, ".crates");

const program = "resolver";
const programName = "oracle_resolver";

await generateIdl({
  generator: "shank",
  programName,
  idlName: programName,
  idlDir,
  binaryInstallDir,
  programDir: path.join(programDir, program),
  idlHook: (idl) => {
    idl.name = programName;
    return idl;
  },
});
