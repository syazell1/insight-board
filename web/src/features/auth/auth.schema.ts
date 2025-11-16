import z from "zod";

export const loginUserSchema = z.object({
  username: z.string(),
  password: z.string()
})

export const registerUserSchema = z.object({
  username: z.string(),
  email: z.email(),
  password: z.string(),
  name: z.string()
})
