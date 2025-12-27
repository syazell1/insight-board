import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Controller, type Control, type UseFormRegister } from "react-hook-form";
import type { TApiServiceForm } from "../api-service.types";

type ApiServiceFormProps = {
  register: UseFormRegister<TApiServiceForm>,
  control: Control<TApiServiceForm>
}

const ApiServiceForm = ({ register, control }: ApiServiceFormProps) => {
  return (
    <div className="grid gap-4 py-4">
      <div className="grid gap-2">
        <Label htmlFor="name">Name</Label>
        <Input
          id="name"
          placeholder="User API"
          {...register("name")}
        />
      </div>
      <div className="grid gap-2">
        <Label htmlFor="endpoint">Endpoint</Label>
        <Input
          id="endpoint"
          placeholder="/api/users"
          {...register("url")}
        />
      </div>
      <div className="grid gap-2">
        <Label htmlFor="method">Method</Label>
        <Select
          defaultValue="GET"
        >
          <SelectTrigger disabled>
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="GET">GET</SelectItem>
            <SelectItem value="POST">POST</SelectItem>
            <SelectItem value="PUT">PUT</SelectItem>
            <SelectItem value="DELETE">DELETE</SelectItem>
            <SelectItem value="PATCH">PATCH</SelectItem>
          </SelectContent>
        </Select>
      </div>
      <div className="grid gap-2">
        <Label htmlFor="interval">Check Interval</Label>
        <Input
          id="interval"
          placeholder="30"
          {...register("interval_secs", { valueAsNumber: true })}
        />
      </div>
      <div className="grid gap-2">
        <Label htmlFor="description">Description</Label>
        <Input
          id="description"
          placeholder="Get all users"
        />
      </div>
      <div className="grid gap-2">
        <Label htmlFor="status">Status</Label>

        <Controller
          control={control}
          name="is_active"
          render={({ field: { onChange, value } }) => (
            <Select
              onValueChange={e => {
                console.log(e);
                onChange(e === "active" ? true : false);
              }}
              value={value ? "active" : "inactive"}
            >
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="active">Active</SelectItem>
                <SelectItem value="inactive">Inactive</SelectItem>
              </SelectContent>
            </Select>
          )}
        />
      </div>
    </div>
  )
}

export default ApiServiceForm;
