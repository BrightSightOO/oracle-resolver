/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import type { ResolvedAccount, ResolvedAccountsWithIndices } from "../shared";
import type { Context, Pda, PublicKey, Signer, TransactionBuilder } from "@metaplex-foundation/umi";
import type { Serializer } from "@metaplex-foundation/umi/serializers";

import { transactionBuilder } from "@metaplex-foundation/umi";
import { mapSerializer, struct, u8 } from "@metaplex-foundation/umi/serializers";

import { getAccountMetasAndSigners } from "../shared";

// Accounts.
export type CreateV1InstructionAccounts = {
  /** Resolver */
  resolver: PublicKey | Pda;
  /** Market */
  market: PublicKey | Pda;
  /** Oracle request */
  request: PublicKey | Pda;
  /** Payer */
  payer?: Signer;
  /** System program */
  systemProgram?: PublicKey | Pda;
};

// Data.
export type CreateV1InstructionData = { discriminator: number };

export type CreateV1InstructionDataArgs = {};

export function getCreateV1InstructionDataSerializer(): Serializer<
  CreateV1InstructionDataArgs,
  CreateV1InstructionData
> {
  return mapSerializer<CreateV1InstructionDataArgs, any, CreateV1InstructionData>(
    struct<CreateV1InstructionData>([["discriminator", u8()]], {
      description: "CreateV1InstructionData",
    }),
    (value) => ({ ...value, discriminator: 0 }),
  );
}

// Instruction.
export function createV1(
  context: Pick<Context, "payer" | "programs">,
  input: CreateV1InstructionAccounts,
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
      isWritable: true as boolean,
      value: input.resolver ?? null,
    },
    market: {
      index: 1,
      isWritable: false as boolean,
      value: input.market ?? null,
    },
    request: {
      index: 2,
      isWritable: false as boolean,
      value: input.request ?? null,
    },
    payer: {
      index: 3,
      isWritable: true as boolean,
      value: input.payer ?? null,
    },
    systemProgram: {
      index: 4,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Default values.
  if (!resolvedAccounts.payer.value) {
    resolvedAccounts.payer.value = context.payer;
  }
  if (!resolvedAccounts.systemProgram.value) {
    resolvedAccounts.systemProgram.value = context.programs.getPublicKey(
      "splSystem",
      "11111111111111111111111111111111",
    );
    resolvedAccounts.systemProgram.isWritable = false;
  }

  // Accounts in order.
  const orderedAccounts: Array<ResolvedAccount> = Object.values(resolvedAccounts).sort(
    (a, b) => a.index - b.index,
  );

  // Keys and Signers.
  const [keys, signers] = getAccountMetasAndSigners(orderedAccounts, "programId", programId);

  // Data.
  const data = getCreateV1InstructionDataSerializer().serialize({});

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}