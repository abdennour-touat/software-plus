import { Key } from "../bindings/Key";
import { KeyForCreate } from "../bindings/KeyForCreate";
import { ModelMutateResultData } from "../bindings/ModelMutateResultData";
import { NewUser } from "../bindings/NewUser";
import { User } from "../bindings/User";
import { ipc_invoke } from "../utils/ipc";
import { ensure_ModelMutateResultData } from "../utils/typeAssert";

class BaseFmc<M extends object, C extends object, U extends object> {
  #cmd_suffix: string;
  get cmd_suffix() {
    return this.#cmd_suffix;
  }

  constructor(cmd_suffix: string) {
    this.#cmd_suffix = cmd_suffix;
  }

  async get(id: string): Promise<M> {
    return ipc_invoke(`get_${this.#cmd_suffix}`, { id }).then(
      (res) => res.data
    );
  }

  async add(data: C): Promise<ModelMutateResultData> {
    return ipc_invoke(`add_${this.#cmd_suffix}`, data).then((res) => {
      return ensure_ModelMutateResultData(res.data);
    });
  }

  async update(id: string, data: U): Promise<ModelMutateResultData> {
    return ipc_invoke(`update_${this.#cmd_suffix}`, { id, data }).then(
      (res) => {
        return ensure_ModelMutateResultData(res.data);
      }
    );
  }

  async delete(id: string): Promise<ModelMutateResultData> {
    return ipc_invoke(`delete_${this.#cmd_suffix}`, { id }).then(
      (res) => res.data
    );
  }
}

class KeyFmc extends BaseFmc<Key, KeyForCreate, BigInteger> {
  constructor() {
    super("key");
  }
  async check() {
    return await ipc_invoke(`check_${this.cmd_suffix}`);
  }
  async addKey(data: KeyForCreate): Promise<String> {
    return await ipc_invoke(`add_${this.cmd_suffix}`, data);
  }
}
class UserFmc extends BaseFmc<User, NewUser, BigInteger> {
  constructor() {
    super("user");
  }
  async check() {
    return await ipc_invoke(`check_${this.cmd_suffix}`);
  }
  async auth() {
    return await ipc_invoke(`check_auth`);
  }
}
export const keyFmc = new KeyFmc();
export const userFmc = new UserFmc();
