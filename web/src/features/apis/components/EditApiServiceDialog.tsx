import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { useEffect, useState } from "react";
import { useForm } from "react-hook-form";
import type { TApiServiceForm } from "../api-service.types";
import { zodResolver } from "@hookform/resolvers/zod";
import { apiServiceFormSchema } from "../api-service.schemas";
import { Plus } from "lucide-react";
import { Button } from "@/components/ui/button";
import { useEditApi } from "../hooks/useEditApi";
import ApiServiceForm from "./ApiServiceForm";

const EditApiServiceDialog = () => {
  const [isOpen, setIsOpen] = useState(false)
  const { register, control, handleSubmit, formState: { errors } } = useForm<TApiServiceForm>({
    resolver: zodResolver(apiServiceFormSchema)
  });

  const { mutate, isSuccess, isPending } = useEditApi();

  useEffect(() => {
    if (isSuccess) {
      setIsOpen(false);
    }
  }, [isSuccess])

  const submitHandler = (data: TApiServiceForm) => {
    mutate(data);
  }

  return (
    <Dialog open={isOpen} onOpenChange={setIsOpen}>
      <DialogTrigger asChild>
        <Button>
          <Plus className="mr-2 h-4 w-4" />
          Add Service
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Add New Service</DialogTitle>
          <DialogDescription>
            Create a new API endpoint or service
          </DialogDescription>
        </DialogHeader>
        <ApiServiceForm register={register} control={control} />
        <DialogFooter>
          <DialogClose asChild>
            <Button variant="outline" onClick={() => { }}>
              Cancel
            </Button>
          </DialogClose>
          <Button onClick={handleSubmit(submitHandler)}>
            {isPending ? "Adding..." : "Add Service"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}

export default EditApiServiceDialog;
