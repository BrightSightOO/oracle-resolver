import type { UmiPlugin } from "@metaplex-foundation/umi";

import { createOracleResolverProgram } from "./generated";

export const oracleResolver = (): UmiPlugin => ({
  install(umi) {
    umi.programs.add(createOracleResolverProgram(), false);
  },
});
