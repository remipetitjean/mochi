FROM golang:1.17 AS build-env

ENV CGO_ENABLED=0
ENV GOOS=linux

WORKDIR $GOPATH/src/mochi
COPY ./src .

RUN go mod download

RUN go build -o migrate cmd/migrate/main.go 
RUN go build -o load_stocks cmd/load_stocks/main.go 
RUN go build -o load_previous_day_stock_prices cmd/load_previous_day_stock_prices/main.go 
RUN go build -o load_stock_prices_since_inception cmd/load_stock_prices_since_inception/main.go 
RUN go build -o mochi main.go

#FROM scratch
#COPY --from=build-env /go/src/mochi/server .
#COPY --from=build-env /go/src/mochi/load_prices_since_inception .
#COPY --from=build-env /go/src/mochi/load_previous_prices .
#COPY --from=build-env /go/src/mochi/migrate .
#EXPOSE 8090
#CMD ["./server"]

FROM ubuntu:22.04

# install ssl certificates
RUN apt update
RUN apt install -y ca-certificates
RUN update-ca-certificates

COPY --from=build-env /go/src/mochi/migrate .
COPY --from=build-env /go/src/mochi/load_stocks .
COPY --from=build-env /go/src/mochi/load_previous_day_stock_prices .
COPY --from=build-env /go/src/mochi/load_stock_prices_since_inception .
COPY --from=build-env /go/src/mochi/mochi .

EXPOSE 8080
CMD ["./mochi"]
