import { defaultErrorMessage, ErrorCodes } from "./constants";
import type { ErrorCode, StandardErrorOptions } from "./types";

const standardRpcError = {
  invalidInput: <T>(options: StandardErrorOptions<T>) => {
    return {
      code: ErrorCodes.invalidInput,
      message: options.message ?? defaultErrorMessage[ErrorCodes.invalidInput],
      data: options.data,
    };
  },
  resourceNotFound: <T>(options: StandardErrorOptions<T>) => {
    return {
      code: ErrorCodes.resourceNotFound,
      message:
        options.message ?? defaultErrorMessage[ErrorCodes.resourceNotFound],
      data: options.data,
    };
  },
  resourceUnavailable: <T>(options: StandardErrorOptions<T>) => {
    return {
      code: ErrorCodes.resourceUnavailable,
      message:
        options.message ?? defaultErrorMessage[ErrorCodes.resourceUnavailable],
      data: options.data,
    };
  },
  transactionRejected: <T>(options: StandardErrorOptions<T>) => {
    return {
      code: ErrorCodes.transactionRejected,
      message:
        options.message ?? defaultErrorMessage[ErrorCodes.transactionRejected],
      data: options.data,
    };
  },
  methodNotSupported: <T>(options: StandardErrorOptions<T>) => {
    return {
      code: ErrorCodes.methodNotSupported,
      message:
        options.message ?? defaultErrorMessage[ErrorCodes.methodNotSupported],
      data: options.data,
    };
  },
  limitExceeded: <T>(options: StandardErrorOptions<T>) => {
    return {
      code: ErrorCodes.limitExceeded,
      message: options.message ?? defaultErrorMessage[ErrorCodes.limitExceeded],
      data: options.data,
    };
  },
  versionNotSupported: <T>(options: StandardErrorOptions<T>) => {
    return {
      code: ErrorCodes.versionNotSupported,
      message:
        options.message ?? defaultErrorMessage[ErrorCodes.versionNotSupported],
      data: options.data,
    };
  },
  invalidRequest: <T>(options: StandardErrorOptions<T>) => {
    return {
      code: ErrorCodes.invalidRequest,
      message:
        options.message ?? defaultErrorMessage[ErrorCodes.invalidRequest],
      data: options.data,
    };
  },
  methodNotFound: <T>(options: StandardErrorOptions<T>) => {
    return {
      code: ErrorCodes.methodNotFound,
      message:
        options.message ?? defaultErrorMessage[ErrorCodes.methodNotFound],
      data: options.data,
    };
  },
  invalidParams: <T>(options: StandardErrorOptions<T>) => {
    return {
      code: ErrorCodes.invalidParams,
      message: options.message ?? defaultErrorMessage[ErrorCodes.invalidParams],
      data: options.data,
    };
  },
  internal: <T>(options: StandardErrorOptions<T>) => {
    return {
      code: ErrorCodes.internal,
      message: options.message ?? defaultErrorMessage[ErrorCodes.internal],
      data: options.data,
    };
  },
  parse: <T>(options: StandardErrorOptions<T>) => {
    return {
      code: ErrorCodes.parse,
      message: options.message ?? defaultErrorMessage[ErrorCodes.parse],
      data: options.data,
    };
  },
};

const standardProviderError = {
  userRejectedRequest: <T>(options: StandardErrorOptions<T>) => {
    return {
      code: ErrorCodes.userRejectedRequest,
      message:
        options.message ?? defaultErrorMessage[ErrorCodes.userRejectedRequest],
      data: options.data,
    };
  },
  unauthorized: <T>(options: StandardErrorOptions<T>) => {
    return {
      code: ErrorCodes.unauthorized,
      message: options.message ?? defaultErrorMessage[ErrorCodes.unauthorized],
      data: options.data,
    };
  },
  unsupportedMethod: <T>(options: StandardErrorOptions<T>) => {
    return {
      code: ErrorCodes.unsupportedMethod,
      message:
        options.message ?? defaultErrorMessage[ErrorCodes.unsupportedMethod],
      data: options.data,
    };
  },
  disconnected: <T>(options: StandardErrorOptions<T>) => {
    return {
      code: ErrorCodes.disconnected,
      message: options.message ?? defaultErrorMessage[ErrorCodes.disconnected],
      data: options.data,
    };
  },
  chainDisconnected: <T>(options: StandardErrorOptions<T>) => {
    return {
      code: ErrorCodes.chainDisconnected,
      message:
        options.message ?? defaultErrorMessage[ErrorCodes.chainDisconnected],
      data: options.data,
    };
  },
  unsupportedChain: <T>(options: StandardErrorOptions<T>) => {
    return {
      code: ErrorCodes.unsupportedChain,
      message:
        options.message ?? defaultErrorMessage[ErrorCodes.unsupportedChain],
      data: options.data,
    };
  },
};

export const standardError = {
  ...standardRpcError,
  ...standardProviderError,
  from: <T>(code: ErrorCode, options: StandardErrorOptions<T>) => {
    return {
      code,
      message: options.message ?? defaultErrorMessage[code],
      data: options.data,
    };
  },
};
