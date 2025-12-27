import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useRegisterUser } from "../hooks/useRegisterUser";
import type { TRegisterUser } from "../auth.types";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { registerUserSchema } from "../auth.schema";
import { isAxiosError } from "axios";
import type { IErrorResponse } from "@/models/error-response.types";

const RegisterUserForm = () => {
  const { register, handleSubmit } = useForm<TRegisterUser>({
    resolver: zodResolver(registerUserSchema),
    defaultValues: {
      username: "",
      password: "",
    }
  });

  const { mutate, isPending, error } = useRegisterUser();


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
          placeholder="Username"
          required
        />
      </div>
      <div className="space-y-2">
        <Label htmlFor="signup-password">Password</Label>
        <Input
          {...register("password")}
          id="signup-password"
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
        {isPending ? "Creating account..." : "Create Account"}
      </Button>
    </form>
  )
}

export default RegisterUserForm;
