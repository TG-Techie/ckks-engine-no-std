# Use an official Rust image as the base
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/ckks-engine

# Copy the rest of the project files
COPY . .

# Build the project using Cargo
RUN cargo build

# The default command will be to keep the container running
CMD ["tail", "-f", "/dev/null"]
