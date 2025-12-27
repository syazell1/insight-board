import z from "zod";

export const apiServiceFormSchema = z.object({
    name : z.string(), 
    url : z.string(),
    interval_secs: z.number().min(10).max(86400).optional(),
    is_active : z.boolean().optional(),
})