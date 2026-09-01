FROM golang:1.23-alpine AS build
WORKDIR /src
COPY go.mod ./
COPY cmd ./cmd
RUN go build -trimpath -ldflags='-s -w' -o /out/sentinel-server ./cmd/sentinel-server

FROM alpine:3.20
RUN apk add --no-cache ca-certificates wireguard-tools nftables
COPY --from=build /out/sentinel-server /usr/local/bin/sentinel-server
EXPOSE 8080/udp 8080/tcp
ENTRYPOINT ["/usr/local/bin/sentinel-server"]
