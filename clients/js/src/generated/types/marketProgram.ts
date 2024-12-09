/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import type { Serializer } from "@metaplex-foundation/umi/serializers";

import { scalarEnum } from "@metaplex-foundation/umi/serializers";

export enum MarketProgram {
  P2p,
  LegacyAmm,
  Parimutuel,
  ParimutuelLulo,
}

export type MarketProgramArgs = MarketProgram;

export function getMarketProgramSerializer(): Serializer<MarketProgramArgs, MarketProgram> {
  return scalarEnum<MarketProgram>(MarketProgram, {
    description: "MarketProgram",
  }) as Serializer<MarketProgramArgs, MarketProgram>;
}