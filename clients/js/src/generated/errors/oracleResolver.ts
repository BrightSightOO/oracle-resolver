/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import type { Program } from "@metaplex-foundation/umi";

import { ProgramError } from "@metaplex-foundation/umi";

type ProgramErrorConstructor = new (program: Program, cause?: Error) => ProgramError;
const codeToErrorMap = new Map<number, ProgramErrorConstructor>();
const nameToErrorMap = new Map<string, ProgramErrorConstructor>();

/** DeserializationError: Failed to deserialize account */
export class DeserializationErrorError extends ProgramError {
  override readonly name: string = "DeserializationError";

  readonly code: number = 0x0; // 0

  constructor(program: Program, cause?: Error) {
    super("Failed to deserialize account", program, cause);
  }
}
codeToErrorMap.set(0x0, DeserializationErrorError);
nameToErrorMap.set("DeserializationError", DeserializationErrorError);

/** SerializationError: Failed to serialize account */
export class SerializationErrorError extends ProgramError {
  override readonly name: string = "SerializationError";

  readonly code: number = 0x1; // 1

  constructor(program: Program, cause?: Error) {
    super("Failed to serialize account", program, cause);
  }
}
codeToErrorMap.set(0x1, SerializationErrorError);
nameToErrorMap.set("SerializationError", SerializationErrorError);

/** InvalidRequestKind: Invalid request kind */
export class InvalidRequestKindError extends ProgramError {
  override readonly name: string = "InvalidRequestKind";

  readonly code: number = 0x2; // 2

  constructor(program: Program, cause?: Error) {
    super("Invalid request kind", program, cause);
  }
}
codeToErrorMap.set(0x2, InvalidRequestKindError);
nameToErrorMap.set("InvalidRequestKind", InvalidRequestKindError);

/** RequestNotResolved: Request is not resolved */
export class RequestNotResolvedError extends ProgramError {
  override readonly name: string = "RequestNotResolved";

  readonly code: number = 0x3; // 3

  constructor(program: Program, cause?: Error) {
    super("Request is not resolved", program, cause);
  }
}
codeToErrorMap.set(0x3, RequestNotResolvedError);
nameToErrorMap.set("RequestNotResolved", RequestNotResolvedError);

/**
 * Attempts to resolve a custom program error from the provided error code.
 * @category Errors
 */
export function getOracleResolverErrorFromCode(
  code: number,
  program: Program,
  cause?: Error,
): ProgramError | null {
  const constructor = codeToErrorMap.get(code);
  return constructor ? new constructor(program, cause) : null;
}

/**
 * Attempts to resolve a custom program error from the provided error name, i.e. 'Unauthorized'.
 * @category Errors
 */
export function getOracleResolverErrorFromName(
  name: string,
  program: Program,
  cause?: Error,
): ProgramError | null {
  const constructor = nameToErrorMap.get(name);
  return constructor ? new constructor(program, cause) : null;
}
