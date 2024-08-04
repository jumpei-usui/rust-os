FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

RUN sudo apt update && apt upgrade -y && apt install qemu-system -y \
    && rustup target add x86_64-unknown-uefi && cargo install uefi-run
