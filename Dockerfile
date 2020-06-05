FROM rustlang/rust:nightly AS build
WORKDIR /usr/src
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new ip-monitor-prometheus
WORKDIR /usr/src/ip-monitor-prometheus
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

# We need to setup tini in this build container b/c the scratch container
# doesn't contain chmod
ENV TINI_VERSION v0.19.0
ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini-static /tini
RUN chmod +x /tini

FROM scratch
COPY --from=build /usr/local/cargo/bin/ip-monitor-prometheus .
COPY --from=build /tini /tini
USER 1000
CMD ["./ip-monitor-prometheus"]  
ENTRYPOINT ["/tini", "--"]
