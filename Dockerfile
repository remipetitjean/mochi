FROM golang:1.17 AS build-env

ENV CGO_ENABLED=0
ENV GOOS=linux

WORKDIR $GOPATH/src/position-service
COPY ./src .

RUN go mod download

RUN go build -o migrate cmd/migrate/main.go 
RUN go build -o load_stocks cmd/load_stocks/main.go 
RUN go build -o load_previous_day_stock_prices cmd/load_previous_day_stock_prices/main.go 
RUN go build -o load_stock_prices_since_inception cmd/load_stock_prices_since_inception/main.go 
RUN go build -o position_service main.go

#FROM scratch
#COPY --from=build-env /go/src/position-service/server .
#COPY --from=build-env /go/src/position-service/load_prices_since_inception .
#COPY --from=build-env /go/src/position-service/load_previous_prices .
#COPY --from=build-env /go/src/position-service/migrate .
#EXPOSE 8090
#CMD ["./server"]

FROM ubuntu:22.04

# install ssl certificates
RUN apt update
RUN apt install -y ca-certificates
RUN update-ca-certificates

COPY --from=build-env /go/src/position-service/migrate .
COPY --from=build-env /go/src/position-service/load_stocks .
COPY --from=build-env /go/src/position-service/load_previous_day_stock_prices .
COPY --from=build-env /go/src/position-service/load_stock_prices_since_inception .
COPY --from=build-env /go/src/position-service/position_service .

EXPOSE 8080
CMD ["./position_service"]
