import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useRegisterUser } from "../hooks/useRegisterUser";
import type { TRegisterUser } from "../auth.types";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { registerUserSchema } from "../auth.schema";

const RegisterUserForm = () => {
  const { register, handleSubmit } = useForm<TRegisterUser>({
    resolver: zodResolver(registerUserSchema),
    defaultValues: {
      username: "",
      password: "",
      email: "",
      name: ""
    }
  });

  const { mutate, isPending } = useRegisterUser();


  const submitRegister = (data: TRegisterUser) => {
    mutate(data);
  }

  return (
    <form className="space-y-4" onSubmit={handleSubmit(submitRegister)}>
      <div className="space-y-2">
        <Label htmlFor="username">Username</Label>
        <Input
          {...register("username")}
          id="username"
          type="text"
          placeholder="John Doe"
          required
        />
      </div>
      <div className="space-y-2">
        <Label htmlFor="signup-email">Email</Label>
        <Input
          {...register("email")}
          id="signup-email"
          type="email"
          placeholder="name@example.com"
          required
        />
      </div>
      <div className="space-y-2">
        <Label htmlFor="signup-password">Password</Label>
        <Input
          {...register("password")}
          id="signup-password"
          type="password"
          required
        />
      </div>
      <Button
        type="submit"
        className="w-full bg-primary hover:bg-primary/90"
        disabled={isPending}
      >
        {isPending ? "Creating account..." : "Create Account"}
      </Button>
    </form>
  )
}

export default RegisterUserForm;
