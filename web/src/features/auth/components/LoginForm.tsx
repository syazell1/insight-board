import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { zodResolver } from "@hookform/resolvers/zod";
import { loginUserSchema } from "../auth.schema";
import { useForm } from "react-hook-form";
import type { TLoginUser } from "../auth.types";
import { useLoginUser } from "../hooks/useLoginUser";

const LoginForm = () => {
  const { register, handleSubmit } = useForm<TLoginUser>({
    resolver: zodResolver(loginUserSchema),
    defaultValues: {
      username: "",
      password: ""
    }
  });

  const { mutate, isPending } = useLoginUser()

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
          required
        />
      </div>
      <div className="space-y-2">
        <Label htmlFor="login-password">Password</Label>
        <Input
          {...register("password")}
          id="login-password"
          type="password"
          required
        />
      </div>
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
