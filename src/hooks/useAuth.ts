import { Dispatch, useEffect, useReducer, useState } from "react";
import { AuthData } from "../bindings/AuthData";
import { keyFmc, userFmc } from "../model";

export interface FormProps {
  key: string;
  username: string;
  password: string;
  authed: boolean;
}
export interface ReducerProp {
  state: FormProps;
  dispatch: Dispatch<Action>;
}
export enum ActionKind {
  ADD_KEY = "ADD_KEY",
  ADD_USERNAME = "ADD_USERNAME",
  ADD_PASSWORD = "ADD_PASSWORD",
  SET_AUTHED = "SET_AUTHED",
}
type Action = {
  type: ActionKind;
  payload: string;
};

const initialData: FormProps = {
  key: "",
  username: "",
  password: "",
  authed: false,
};
const reducer = (state: FormProps, action: Action): FormProps => {
  switch (action.type) {
    case "ADD_KEY":
      return {
        ...state,
        key: action.payload,
      };
    case "ADD_USERNAME":
      return {
        ...state,
        username: action.payload,
      };
    case "ADD_PASSWORD":
      return {
        ...state,
        password: action.payload,
      };
    case "SET_AUTHED":
      return {
        ...state,
        authed: true,
      };
    default:
      return { ...state };
  }
};

export function useAuth() {
  const [state, dispatch] = useReducer(reducer, initialData);
  return { state, dispatch };
}
