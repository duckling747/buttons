FROM rust as build
WORKDIR /usr/src/buttons
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=build /usr/local/cargo/bin/buttons /usr/local/bin/buttons
RUN mkdir /usr/src/buttons
COPY --from=build /usr/src/buttons/templates /usr/src/buttons/templates
EXPOSE 8000
CMD ["buttons"]
