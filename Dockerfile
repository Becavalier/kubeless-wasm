# FROM bitnami/minideb:latest
FROM rust:1.53
RUN apt-get update && apt-get install -y vim
USER 1000
WORKDIR /kubeless
CMD ["./server"]
