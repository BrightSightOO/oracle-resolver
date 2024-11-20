// @ts-check

const path = require("node:path");

const rootDir = path.dirname(__dirname);
const binDir = path.join(rootDir, ".bin");

/**
 * @param {string} binary
 * @returns {string}
 */
function getProgram(binary) {
  return path.join(binDir, binary);
}

/** @type {import("@metaplex-foundation/amman").AmmanConfig} */
module.exports = {
  validator: {
    matchFeatures: "mainnet-beta",
    commitment: "processed",
    accountsCluster: "https://api.mainnet-beta.solana.com",
    programs: [
      {
        label: "Oracle Resolver",
        programId: "RESwds5X9Yj1kzXkjuA5ncR8TqhHeqj7qcrUz9QM29f",
        deployPath: getProgram("resolver_program.so"),
      },
    ],
  },
};
