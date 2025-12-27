import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog";
import { Button } from "@/components/ui/button";
import { Trash2 } from "lucide-react";
import { useEffect, useState, type FC } from "react";
import { useDeleteApi } from "../hooks/useDeleteApi";

type DeleteApiDialogProps = {
  id: string
}

const DeleteApiDialog: FC<DeleteApiDialogProps> = ({ id }) => {
  const [isOpen, setIsOpen] = useState(false);
  const { mutate, isPending, isSuccess } = useDeleteApi()

  useEffect(() => {
    if (isSuccess) {
      setIsOpen(false)
    }
  }, [isSuccess])

  const deleteHandler = () => {
    mutate(id);
  }

  return (
    <AlertDialog open={isOpen} onOpenChange={setIsOpen} >
      <AlertDialogTrigger asChild>
        <Button
          variant="ghost"
          size="icon"
          onClick={() => { }}
        >
          <Trash2 className="h-4 w-4 text-destructive" />
        </Button>
      </AlertDialogTrigger>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>Are you sure?</AlertDialogTitle>
          <AlertDialogDescription>
            This will permanently delete the service
            This action cannot be undone.
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel>Cancel</AlertDialogCancel>
          <AlertDialogAction onClick={deleteHandler}>
            {isPending ? "Loading..." : "Delete"}
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  )
}

export default DeleteApiDialog;
