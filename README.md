# User DDD Example

DDD without repository pattern, see [https://softwareengineering.stackexchange.com/questions/441348/in-the-oo-ddd-does-the-domain-object-access-the-repository-directly]

## Setup

1. Create .env file if needed. Run `docker-compose up -d` to run Postgres in the background.

2. Declare the database URL

    ```console
    export DATABASE_URL="postgres://postgres:password@localhost/users"
    ```

    Access from command line like:

    ```console
    psql postgresql://postgres:password@localhost/users -c 'select * from users'
    ```

3. Create the database.

    ```console
    sqlx db create
    ```

4. Run sql migrations

    ```console
    sqlx migrate run
    ```

## Usage

```console
cargo run
```

## Query DB from shell

```console
psql postgresql://postgres:password@localhost/users -c 'select * from users'
```

