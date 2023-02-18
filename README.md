# Software Plus

_management system for entreprieses and small businesses_

## dependencies:

- diesel: database ORM tool
- dotenv: environment variables
- sqlite: database
- thiserror: error handling
- serde: serialization and deserialization
- serde_json: json serialization and deserialization
- ts_rs: typescript bindings
- license_key: license key generator
- magic_crypt: encryption and decryption

## how to run:

1 - clone the repository
2 - run `yarn` to install dependencies
3 - run `yarn tauri dev` to run the app in development mode

## how to build:

1 - clone the repository
2 - run `yarn` to install dependencies
3 - run `yarn tauri build` to build the app

## how to run tests:

1 - clone the repository
2 - run `yarn` to install dependencies
3 - run `yarn test` to run the tests

## how to run migrations:

1 - clone the repository
2 - run `yarn` to install dependencies
3 - run `diesel migration run` to run the migrations

## how to generate typescript bindings:

1 - clone the repository
2 - run `yarn` to install dependencies
3 - run `yarn tauri build` to build the app
4 - run `yarn tauri info` to get the path to the tauri folder
5 - run `cd src-tauri && cargo test` to generate the bindings
