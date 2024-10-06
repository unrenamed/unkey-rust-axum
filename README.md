# Protect your Rust + Axum API with Unkey

This example shows how to secure a Rust + Axum API using [Unkey](https://www.unkey.com/) for API key management. You'll learn how to protect public and private routes with minimal setup and start authenticating users quickly.

## Quickstart

### Create a root key

1. Go to [/settings/root-keys](https://app.unkey.com/settings/root-key) and click on the "Create New Root Key" button.
2. Enter a name for the key.
3. Select the following workspace permissions: `create_key`, `read_key`, `encrypt_key` and `decrypt_key`.
4. Click "Create".

### Create your API

1. Go to [https://app.unkey.com/apis](https://app.unkey.com/apis) and click on the "Create New API" button.
2. Give it a name.
3. Click "Create".

### Create your first API key

1. Click "Create Key" in the top right corner.
2. Feel the form with any key information you want or leave it empty.
3. Click "Create"
4. Copy the key and save it somewhere safe.

### Set up the example

1. Clone the repository to your local machine:

```bash
git clone git@github.com:unrenamed/unkey-rust-axum
cd unkey-rust-axum
```

2. Create a `.env` file in the root directory and populate it with the following environment variables:

```env
PORT=3000
UNKEY_ROOT_KEY=your-unkey-root-key
UNKEY_API_ID=your-unkey-api-id
```

Ensure you replace `your-unkey-root-key` and `your-unkey-api-id` with your actual Unkey credentials.

4. Start the server:

```bash
cargo run
```

The server will start and listen on the port specified in the `.env` file (default is `3000`).

5. Test the public route as a guest:

```bash
  curl http://localhost:3000/public
```

6. Test the public route as an authorized user by passing the API key in the header:

```bash
  curl http://localhost:3000/public -H "Authorization: Bearer <YOUR_API_KEY>"
```

7. Test the protected route, which requires valid authorization:

```bash
  curl http://localhost:3000/protected -H "Authorization: Bearer <YOUR_API_KEY>"
```
