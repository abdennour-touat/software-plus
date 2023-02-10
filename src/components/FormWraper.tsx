import { Typography } from "@mui/material";
import { ReactNode } from "react";

type FormWrapperProps = {
  title: string;
  children: ReactNode;
};

export function FormWrapper({ title, children }: FormWrapperProps) {
  return (
    <>
      <Typography>{title}</Typography>
      <div>{children}</div>
    </>
  );
}
