import { Button, TextField } from "@mui/material";
import { useState } from "react";
import {
  FieldErrors,
  SubmitHandler,
  UseFormHandleSubmit,
  UseFormRegister,
} from "react-hook-form/dist/types";
import { NewUser } from "../bindings/NewUser";
import { ActionKind, ReducerProp } from "../hooks/useAuth";
import { userFmc } from "../model";
import { FormWrapper } from "./FormWraper";

export interface AddUserProps {
  register: UseFormRegister<NewUser>;
  handleSubmit: UseFormHandleSubmit<NewUser>;
  onSubmit: SubmitHandler<NewUser>;
  errors: FieldErrors;
}
export default function AddUser({ dispatch, state }: ReducerProp) {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  async function addUser() {
    await userFmc.add({ password, username });
    setPassword("");
    dispatch({ type: ActionKind.ADD_USERNAME, payload: username });

    dispatch({ type: ActionKind.SET_AUTHED, payload: "" });
    setUsername("");
  }
  return (
    <FormWrapper title="please create a new user">
      <TextField
        value={username}
        onChange={(e) => setUsername(e.target.value)}
        //  {...register("username", { required: true })}
      />
      <TextField
        type="password"
        value={password}
        onChange={(e) => setPassword(e.target.value)}
        //  {...register("password", { required: true })}
      />
      <Button onClick={() => addUser()}> submit</Button>
      {/* <TextField
        // type="password"
        {...(register("password"), { required: true })}
      /> */}
      {/* errors will return when field validation fails  */}
      {/* {errors.exampleRequired && <span>This field is required</span>} */}
    </FormWrapper>
  );
}
