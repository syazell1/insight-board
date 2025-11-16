import z from "zod";
import { loginUserSchema, registerUserSchema } from "./auth.schema";

export interface IAuthResponse {
  id: string,
  access_token: string
}

export type TLoginUser = z.infer<typeof loginUserSchema>;
export type TRegisterUser = z.infer<typeof registerUserSchema>;
