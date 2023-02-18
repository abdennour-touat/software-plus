# Software Plus

_management system for entreprieses and small businesses_

## dependencies

- diesel: database ORM tool
- dotenv: environment variables
- sqlite: database
- thiserror: error handling
- serde: serialization and deserialization
- serde_json: json serialization and deserialization
- ts_rs: typescript bindings
- license_key: license key generator
- magic_crypt: encryption and decryption

## how to run

1. clone the repository
2. run `yarn` to install dependencies
3. run `yarn tauri dev` to run the app in development mode

## how to build

1. clone the repository
2. run `yarn` to install dependencies
3. run `yarn tauri build` to build the app

## how to run tests

1. clone the repository
2. run `yarn` to install dependencies
3. run `yarn test` to run the tests

## how to run migrations

1. clone the repository
2. run `yarn` to install dependencies
3. run `diesel migration run` to run the migrations

## how to generate typescript bindings

1. clone the repository
2. run `yarn` to install dependencies
3. run `yarn tauri build` to build the app
4. run `yarn tauri info` to get the path to the tauri folder
5. run `cd src-tauri && cargo test` to generate the bindings

## API Reference

### Create User

```rust
    UserBmc::insert_user(
        store,
        "username",
        "password",
    )
```

#### Description

Creates a new user in the database

#### Parameters

| Name     | Type         | Description                  |
| -------- | ------------ | ---------------------------- |
| store    | `ConnPooled` | a refrence to the connection |
| username | `String`     | the username                 |
| password | `String `    | the password                 |

#### Returns

| Type         | Description               |
| ------------ | ------------------------- |
| `Result<()>` | the user that was created |

#### Errors

| Type                   | Description                          |
| ---------------------- | ------------------------------------ |
| `Error::DatabaseError` | no license key found in the database |

#### Example

```rust
    let store = establish_connection();
    let user = UserBmc::insert_user(
        &store,
        "username",
        "password",
    );
```

### Get User

```rust
    UserBmc::get_user(
        store,
        id,
    )
```

#### Description

Gets a user from the database by id

#### Parameters

| Name  | Type         | Description                  |
| ----- | ------------ | ---------------------------- |
| store | `ConnPooled` | a refrence to the connection |
| id    | `i32`        | the id of the user           |

#### Returns

| Type           | Description          |
| -------------- | -------------------- |
| `Result<User>` | the user in question |

#### Errors

| Type                   | Description   |
| ---------------------- | ------------- |
| `Error::DataBaseError` | no user found |

#### Example

```rust
    let store = establish_connection();
    let user = UserBmc::get_user(
        &store,
        1,
    );
```

### get users

```rust
    UserBmc::get_users(
        store,
    )
```

#### Description

Gets all users from the database

#### Parameters

| Name  | Type         | Description                  |
| ----- | ------------ | ---------------------------- |
| store | `ConnPooled` | a refrence to the connection |

#### Returns

| Type                | Description               |
| ------------------- | ------------------------- |
| `Result<Vec<User>>` | all users in the database |

#### Errors

| Type                   | Description   |
| ---------------------- | ------------- |
| `Error::DataBaseError` | no user found |

#### Example

```rust
    let store = establish_connection();
    let users = UserBmc::get_users(
        &store,
    );
```

### Delete User

```rust
    UserBmc::delete(
        store,
        user_id,
    )
```

#### Description

Deletes a user from the database

#### Parameters

| Name    | Type         | Description                  |
| ------- | ------------ | ---------------------------- |
| store   | `ConnPooled` | a refrence to the connection |
| user_id | `i32`        | the id of the user           |

#### Returns

| Type         | Description               |
| ------------ | ------------------------- |
| `Result<()>` | the user that was deleted |

#### Errors

| Type                   | Description          |
| ---------------------- | -------------------- |
| `Error::DataBaseError` | couldn't delete user |

#### Example

```rust
    let store = establish_connection();
    let res = UserBmc::delete(
        &store,
        1,
    );
```

### delete all users

```rust
    UserBmc::delete_all(
        store,
    )
```

#### Description

Deletes all users from the database

#### Parameters

| Name  | Type         | Description                  |
| ----- | ------------ | ---------------------------- |
| store | `ConnPooled` | a refrence to the connection |

#### Returns

| Type           | Description           |
| -------------- | --------------------- |
| `Result<bool>` | all the users deleted |

#### Errors

| Type                   | Description           |
| ---------------------- | --------------------- |
| `Error::DataBaseError` | couldn't delete users |

#### Example

```rust
    let store = establish_connection();
    let res = UserBmc::delete_all(
        &store,
    );
```

### Check Authentification

```rust
    UserBmc::check_auth(
        store,
    )
```

#### Description

check whether the key is available in the database

#### Parameters

| Name  | Type         | Description                  |
| ----- | ------------ | ---------------------------- |
| store | `ConnPooled` | a refrence to the connection |

#### Returns

| Type               | Description    |
| ------------------ | -------------- |
| `Result<AuthData>` | the auth state |

#### Errors

| Type       | Description      |
| ---------- | ---------------- |
| `AuthData` | auth state error |

#### Example

```rust
    let store = establish_connection();
    let res = UserBmc::check_authentication(
        &store,
    );
```

### create license key

```rust
    LicenseKey::create_license_key(
        store,
        hash,
    )
```

#### Description

creates a new license key in the database

#### Parameters

| Name  | Type         | Description                  |
| ----- | ------------ | ---------------------------- |
| store | `ConnPooled` | a refrence to the connection |
| hash  | `String`     | the hash of the key          |

#### Returns

| Type     | Description                                           |
| -------- | ----------------------------------------------------- |
| `Status` | the key status (`valid`,`Invalid`,`blocked`,`Forged`) |

#### Example

```rust
    let store = establish_connection();
    let res = LicenseKey::create_license_key(
        &store,
        "hash",
    );
```

### get license key

```rust
    LicenseKey::get(
        store,
    )
```

#### Description

get all the license keys (supposed to be only one)

#### Parameters

| Name  | Type         | Description                  |
| ----- | ------------ | ---------------------------- |
| store | `ConnPooled` | a refrence to the connection |

#### Returns

| Type                       | Description     |
| -------------------------- | --------------- |
| `Result<Vec\<LicenseKey>>` | the license key |

#### Errors

| Type                   | Description              |
| ---------------------- | ------------------------ |
| `Error::DataBaseError` | couldn't get license key |

#### Example

```rust
    let store = establish_connection();
    let res = LicenseKey::get(
        &store,
    );
```

### Delete All license keys

```rust
    LicenseKey::delete_all(
        store,
    )
```

#### Description

Deletes all license keys from the database

#### Parameters

| Name  | Type         | Description                  |
| ----- | ------------ | ---------------------------- |
| store | `ConnPooled` | a refrence to the connection |

#### Returns

| Type           | Description                  |
| -------------- | ---------------------------- |
| `Result<bool>` | all the license keys deleted |

#### Errors

| Type                   | Description                  |
| ---------------------- | ---------------------------- |
| `Error::DataBaseError` | couldn't delete license keys |

#### Example

```rust
    let store = establish_connection();
    let res = LicenseKey::delete_all(
        &store,
    );
```
