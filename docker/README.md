Bloom Service Docker Container

This project contains a Dockerfile to install and run the bloomsrv crate from crates.io.

Prerequisites

Docker installed on your machine.

Build the Image

Run the following command in the directory containing the Dockerfile:

docker build -t bloomsrv-image .


Note: If the crate bloomsrv does not exist on crates.io (e.g. if it was a typo for bloom-server), the build step RUN cargo install... will fail. You can override the crate name using build arguments:

# Example: Building for 'bloom-server' instead
docker build --build-arg CRATE_NAME=bloom-server -t bloomsrv-image .


Run the Container

Once built, you can run the service. You will likely need to map the exposed port to your host machine.

# Replace 8080 with the specific port the application listens on
docker run -p 8080:8080 --name my-bloom-service bloomsrv-image


Environment Variables

If the service requires configuration via environment variables (common for Rust services), pass them using the -e flag:

docker run -p 8080:8080 -e RUST_LOG=info -e HOST=0.0.0.0 bloomsrv-image


Troubleshooting

Crate Not Found: If cargo install bloomsrv fails, verify the exact crate name on crates.io.

Binary Name Mismatch: If the crate name is bloomsrv but the executable binary it installs has a different name, you may need to adjust the COPY command in the Dockerfile to point to the correct binary name in /usr/local/bin/.

Missing Dependencies: If the container starts but crashes immediately with "library not found" errors, the binary might be dynamically linking to system libraries missing in debian:bookworm-slim. You may need to add packages (like libpq5, libsqlite3-0, etc.) to the apt-get install block in the Runtime stage.