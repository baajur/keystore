FROM debian:stable-slim

ARG env=debug

RUN mkdir -p /app/config \
  && mkdir -p /app/migrations \
  && mkdir -p /usr/local/cargo/bin/ \
  && apt-get update \
  && apt-get install -y wget gnupg2 ca-certificates \
  && sh -c 'wget -q https://www.postgresql.org/media/keys/ACCC4CF8.asc -O - | apt-key add -' \
  && sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt/ stretch-pgdg main" >> /etc/apt/sources.list.d/pgdg.list' \
  && wget -q https://s3.eu-central-1.amazonaws.com/dumpster.stq/diesel -O /usr/local/cargo/bin/diesel \
  && chmod +x /usr/local/cargo/bin/diesel \
  && apt-get update \
  && apt-get install -y libpq5 libmariadbclient18 \
  && apt-get purge -y wget \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/ \
  && adduser --disabled-password --gecos "" --home /app --no-create-home -u 5000 app \
  && chown -R app: /app

COPY target/$env/keystore /app
COPY config /app/config
COPY migrations /app/migrations
COPY Cargo.toml /app/Cargo.toml

USER app
WORKDIR /app

ENV PATH=$PATH:/usr/local/cargo/bin/
EXPOSE 8000

ENTRYPOINT ["sh", "-c", "diesel migration run && /app/keystore server"]
