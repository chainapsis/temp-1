export type ErrorCode = number;

export type ErrorObject<T> = {
  code: ErrorCode;
  message: string;
  data?: T;
};

export interface StandardErrorOptions<T> {
  message?: string;
  data?: T;
}
