import { Typography } from "@mui/material";
import { useEffect, useState } from "react";
import AddKey from "../components/AddKey";
import AddUser from "../components/AddUser";
import { useAuth } from "../hooks/useAuth";
import { userFmc } from "../model";
import { AuthData } from "../bindings/AuthData";
import { useNavigate } from "react-router-dom";

function Auth() {
  const { dispatch, state } = useAuth();
  const [auth, setAuth] = useState<AuthData>("NoLicense");
  const navigate = useNavigate();

  async function checkAuth() {
    let res = await userFmc.auth();
    setAuth(res.data);
  }
  console.log(state);
  useEffect(() => {
    checkAuth();
  }, [state]);

  switch (auth) {
    case "NoLicense":
      return <AddKey dispatch={dispatch} state={state} />;
    case "NoUser":
      return <AddUser dispatch={dispatch} state={state} />;
    case "Valid":
      navigate("/login");
    default:
      return <Typography>corrupted version :/</Typography>;
  }
}

export default Auth;
