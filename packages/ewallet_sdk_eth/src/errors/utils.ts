import { defaultErrorMessage, ErrorCodes } from "./constants";
import type { ErrorCode, ErrorObject } from "./types";

const validErrorCodesSet = new Set(Object.values(ErrorCodes));

export const isValidErrorCode = (code: unknown): code is ErrorCode => {
  if (typeof code === "string") {
    try {
      const parsedCode = Number(code);
      return validErrorCodesSet.has(parsedCode);
    } catch (_) {
      return false;
    }
  }

  if (typeof code === "number") {
    return validErrorCodesSet.has(code);
  }

  return false;
};

export const parseError = (
  error: unknown,
): ErrorObject<unknown> | undefined => {
  if (typeof error === "object" && error !== null) {
    if ("code" in error && isValidErrorCode(error.code)) {
      return {
        code: error.code,
        message: defaultErrorMessage[error.code],
        data: "data" in error ? error.data : undefined,
      };
    }
  }

  if (typeof error === "string") {
    return {
      code: ErrorCodes.internal,
      message: error,
    };
  }

  return undefined;
};
