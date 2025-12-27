import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { zodResolver } from "@hookform/resolvers/zod";
import { loginUserSchema } from "../auth.schema";
import { useForm } from "react-hook-form";
import type { TLoginUser } from "../auth.types";
import { useLoginUser } from "../hooks/useLoginUser";
import { isAxiosError } from "axios";
import type { IErrorResponse } from "@/models/error-response.types";

const LoginForm = () => {
  const { register, handleSubmit } = useForm<TLoginUser>({
    resolver: zodResolver(loginUserSchema),
    defaultValues: {
      username: "",
      password: ""
    }
  });

  const { mutate, isPending, error } = useLoginUser()

  const submitLogin = (data: TLoginUser) => {
    mutate(data);
  }

  return (
    <form onSubmit={handleSubmit(submitLogin)} className="space-y-4">
      <div className="space-y-2">
        <Label htmlFor="username">Username</Label>
        <Input
          {...register("username")}
          id="username"
          placeholder="Username"
          required
        />
      </div>
      <div className="space-y-2">
        <Label htmlFor="login-password">Password</Label>
        <Input
          {...register("password")}
          id="login-password"
          type="password"
          placeholder="Password"
          required
        />
      </div>
      {isAxiosError<IErrorResponse>(error) && <p className="text-red-500 text-center text-sm font-medium">{error.response?.data.details}</p>}
      <Button
        type="submit"
        className="w-full bg-primary hover:bg-primary/90"
        disabled={isPending}
      >
        {isPending ? "Signing in..." : "Sign In"}
      </Button>
    </form>
  )
}

export default LoginForm;
