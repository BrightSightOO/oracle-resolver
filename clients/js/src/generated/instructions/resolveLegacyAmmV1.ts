/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import type { ResolvedAccount, ResolvedAccountsWithIndices } from "../shared";
import type { Context, Pda, PublicKey, TransactionBuilder } from "@metaplex-foundation/umi";
import type { Serializer } from "@metaplex-foundation/umi/serializers";

import { transactionBuilder } from "@metaplex-foundation/umi";
import { mapSerializer, struct, u8 } from "@metaplex-foundation/umi/serializers";

import { findResolverV1Pda } from "../accounts";
import { expectPublicKey, getAccountMetasAndSigners } from "../shared";

// Accounts.
export type ResolveLegacyAmmV1InstructionAccounts = {
  /** Resolver */
  resolver?: PublicKey | Pda;
  /** Market */
  market: PublicKey | Pda;
  /** Oracle request */
  request: PublicKey | Pda;
  /** Legacy outcome tokens program */
  outcomeTokensProgram: PublicKey | Pda;
};

// Data.
export type ResolveLegacyAmmV1InstructionData = { discriminator: number };

export type ResolveLegacyAmmV1InstructionDataArgs = {};

export function getResolveLegacyAmmV1InstructionDataSerializer(): Serializer<
  ResolveLegacyAmmV1InstructionDataArgs,
  ResolveLegacyAmmV1InstructionData
> {
  return mapSerializer<
    ResolveLegacyAmmV1InstructionDataArgs,
    any,
    ResolveLegacyAmmV1InstructionData
  >(
    struct<ResolveLegacyAmmV1InstructionData>([["discriminator", u8()]], {
      description: "ResolveLegacyAmmV1InstructionData",
    }),
    (value) => ({ ...value, discriminator: 2 }),
  );
}

// Instruction.
export function resolveLegacyAmmV1(
  context: Pick<Context, "eddsa" | "programs">,
  input: ResolveLegacyAmmV1InstructionAccounts,
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    "oracleResolver",
    "RESwds5X9Yj1kzXkjuA5ncR8TqhHeqj7qcrUz9QM29f",
  );

  // Accounts.
  const resolvedAccounts = {
    resolver: {
      index: 0,
      isWritable: false as boolean,
      value: input.resolver ?? null,
    },
    market: {
      index: 1,
      isWritable: true as boolean,
      value: input.market ?? null,
    },
    request: {
      index: 2,
      isWritable: false as boolean,
      value: input.request ?? null,
    },
    outcomeTokensProgram: {
      index: 3,
      isWritable: false as boolean,
      value: input.outcomeTokensProgram ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Default values.
  if (!resolvedAccounts.resolver.value) {
    resolvedAccounts.resolver.value = findResolverV1Pda(context, {
      market: expectPublicKey(resolvedAccounts.market.value),
      request: expectPublicKey(resolvedAccounts.request.value),
    });
  }

  // Accounts in order.
  const orderedAccounts: Array<ResolvedAccount> = Object.values(resolvedAccounts).sort(
    (a, b) => a.index - b.index,
  );

  // Keys and Signers.
  const [keys, signers] = getAccountMetasAndSigners(orderedAccounts, "programId", programId);

  // Data.
  const data = getResolveLegacyAmmV1InstructionDataSerializer().serialize({});

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}