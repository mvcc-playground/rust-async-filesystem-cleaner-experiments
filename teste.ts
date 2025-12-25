import { z } from "npm:zod@4";


const envSchema = z.object({
    NODE_ENV: z.enum(["development", "test", "production"]),
})

const env = envSchema.safeParse(process.env)
console.log(env.success)