FROM rust:1.83-slim-bookworm as build-phase

RUN apt-get update && apt-get -y upgrade && \
    apt-get -y install libpq-dev pkg-config libssl-dev protobuf-compiler perl make

WORKDIR /app

RUN rm -rf /app/src
RUN rm -rf /app/target/release/deps/graphql-server*
ADD src /app/src
ADD Cargo.toml /app/
ADD Cargo.lock /app/

RUN cargo update -p time
RUN RUST_BACKTRACE=1 cargo build --release

## ------------------------------------------------------------------------------
## Final Stage
## ------------------------------------------------------------------------------

FROM ubuntu:22.04
RUN apt-get update && apt-get -y upgrade && \
    apt-get -y install libpq-dev openssl ca-certificates libssl-dev software-properties-common

RUN add-apt-repository ppa:chris-needham/ppa
RUN apt-get update && apt-get -y install audiowaveform

WORKDIR /app
COPY --from=build-phase /app/target/release/graphql-server /app
RUN mkdir /app/tmp_data
ADD authorization.polar /app
ADD email_templates /app/email_templates

ENV RUST_BACKTRACE=full

EXPOSE 8000
CMD ["./graphql-server"]
