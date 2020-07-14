FROM golang:latest AS builder

WORKDIR /app
ADD ./ .
RUN make

FROM mysql:latest
COPY --from=builder /app/isucari    /app/isucari
