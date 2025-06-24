FROM rust:1.87.0-slim-bullseye AS build

# View app name in Cargo.toml
ARG APP_NAME=motivate-me

RUN apt-get update

WORKDIR /build

COPY Settings.toml Settings.toml
COPY Cargo.toml ./
RUN mkdir src \
    && echo "// dummy file" > src/lib.rs \
    && cargo build --release

COPY src src
RUN cargo build --locked --release
RUN cp ./target/release/$APP_NAME /bin/motivate-me

FROM debian:bullseye-slim AS final

RUN apt-get update

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "10001" \
    appuser
USER appuser

COPY --from=build /bin/motivate-me /bin/
CMD ["/bin/motivate-me"]
