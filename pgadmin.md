# pgAdmin Connection Guide

This guide provides instructions on how to connect to the PostgreSQL database using pgAdmin.

## Connection Details

- **Host name/address**: `db` (This is the service name defined in `docker-compose.yml`)
- **Port**: `5432`
- **Maintenance database**: `postgres`
- **Username**: `postgres`
- **Password**: `password`

## Connection Steps

1. Open your web browser and go to `http://localhost:5050`. This is where pgAdmin is running.
2. Log in to pgAdmin using the default credentials:
   - **Email**: `admin@admin.com`
   - **Password**: `admin`
3. Once logged in, in the left sidebar, right-click on "Servers" and select "Create" > "Server...".
4. In the "Create - Server" dialog:
   - Go to the "General" tab:
     - **Name**: Give your connection a name, e.g., `RustMicro DB`.
   - Go to the "Connection" tab:
     - **Host name/address**: Enter `db`.
     - **Port**: Enter `5432`.
     - **Maintenance database**: Enter `postgres`.
     - **Username**: Enter `postgres`.
     - **Password**: Enter `password`.
     - Check the box "Save password?" if you want pgAdmin to remember the password.
   - Click the "Save" button.
5. You should now see the database server in the left sidebar. Click on it to expand and see the databases.

## Troubleshooting

- If you cannot connect, make sure all Docker containers are running:
  ```bash
  docker-compose ps
  ```
  You should see `rustmicro_db`, `rustmicro_pgadmin`, and `rustmicro_app` all in the "Up" state.

- If you're on Windows or macOS and `db` hostname doesn't work, you can try using the IP address of the database container. Find the IP address by running:
  ```bash
  docker inspect rustmicro_db | grep IPAddress
  ```
  Then use that IP address in the "Host name/address" field instead of `db`.