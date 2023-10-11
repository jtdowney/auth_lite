# auth_lite

`auth_lite` is a lightweight authentication server tailored for web servers that utilize features like the nginx `auth_request` module. Instead of relying on traditional `.htpasswd` files, `auth_lite` employs a nimble SQLite database, making the management of credentials more efficient and seamless.


## Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/jtdowney/auth_lite.git
    cd auth_lite
    ```

2. Build the project:

    ```bash
    cargo build --release
    ```

3. The compiled binary will be available in the `target/release` directory.

## Setup

1. Add a user:

    ```bash
    ./auth_lite add-user USERNAME
    ```

    > This will prompt you for a password. Passwords are hashed and stored securely.

2. Setup nginx:

    In your nginx configuration, incorporate the `auth_request` directive:

    ```nginx
    location /protected/ {
        auth_request /_auth;
        ...
    }

    location = /_auth {
        internal;
        proxy_pass http://127.0.0.1:YOUR_AUTH_LITE_PORT/auth;
    }
    ```

    Make sure to replace `YOUR_AUTH_LITE_PORT` with the port `auth_lite` is listening on.

3. Start the `auth_lite` server:

    ```bash
    ./auth_lite serve --port YOUR_AUTH_LITE_PORT
    ```

## Usage

To manage users in the `auth_lite` SQLite database:

- Add a user:

    ```bash
    ./auth_lite add-user USERNAME
    ```

- List all users:

    ```bash
    ./auth_lite list-users
    ```

- Remove a user:

    ```bash
    ./auth_lite remove-user USERNAME
    ```

- Change a user's password:

    ```bash
    ./auth_lite change-password USERNAME
    ```

## Contributing

Contributions are warmly welcomed! Whether it's bug reports, feature requests, or code contributions, please [open an issue](https://github.com/jtdowney/auth_lite/issues) or a pull request.

## License

`auth_lite` is licensed under the [MIT License](LICENSE).