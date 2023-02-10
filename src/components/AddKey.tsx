import { Box, Button, TextField, Typography } from "@mui/material";
import { useState } from "react";
import { ActionKind, ReducerProp } from "../hooks/useAuth";
import { keyFmc } from "../model";
import { FormWrapper } from "./FormWraper";

export default function AddKey({ dispatch }: ReducerProp) {
  const [key, setkey] = useState("");
  async function addKey() {
    let res: any = await keyFmc.addKey({ hash: key });
    if (res.data == "valid") {
      dispatch({ type: ActionKind.SET_AUTHED, payload: "" });
      setkey("");
    }
  }
  return (
    <FormWrapper title="please insert the key">
      <TextField
        variant="outlined"
        size="small"
        value={key}
        onChange={(e) => setkey(e.target.value)}
      />
      <Button onClick={() => addKey()} variant="contained" color="warning">
        check key
      </Button>
    </FormWrapper>
  );
}
