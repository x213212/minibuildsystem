# Use Rocky Linux 8 as the base image
FROM rockylinux:8

# Install essential packages
RUN dnf -y update && \
    dnf -y install curl gcc gcc-c++ make git python3

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Update PATH for Rust
ENV PATH="/root/.cargo/bin:${PATH}"
