import type z from "zod";
import type { apiServiceFormSchema } from "./api-service.schemas";

export type TApiServiceForm = z.infer<typeof apiServiceFormSchema>;