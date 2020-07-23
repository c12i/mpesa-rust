FROM node:alpine

RUN apk add --no-cache rust cargo
RUN apk add --no-cache openssl-dev

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["mpesa"]
