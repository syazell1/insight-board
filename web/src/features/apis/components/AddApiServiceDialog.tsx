import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Plus } from "lucide-react";
import ApiServiceForm from "./ApiServiceForm";
import { useForm } from "react-hook-form";
import type { TApiServiceForm } from "../api-service.types";
import { zodResolver } from "@hookform/resolvers/zod";
import { apiServiceFormSchema } from "../api-service.schemas";
import { useAddApi } from "../hooks/useAddApi";
import { useEffect, useState } from "react";

const AddApiServiceDialog = () => {
  const [isOpen, setIsOpen] = useState(false)
  const {register, control, handleSubmit, formState: {errors}} = useForm<TApiServiceForm>({
    resolver: zodResolver(apiServiceFormSchema)
  });
  const {mutate, isPending, isSuccess} = useAddApi();

  useEffect(() => {
    if(isSuccess){
      setIsOpen(false);
    }
  }, [isSuccess])

  const submitHandler = (data : TApiServiceForm) => {
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
          <Button variant="outline" onClick={() => { }}>
            Cancel
          </Button>
          <Button onClick={handleSubmit(submitHandler)}>
            {isPending ? "Adding..." : "Add Service"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}

export default AddApiServiceDialog;