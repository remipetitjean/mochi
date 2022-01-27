package main

import (
	"shyrio.com/position-service/core"
	libs "shyrio.com/position-service/libs/iex"
)

func main() {
	core.ConnectDatabase()
	stocks := libs.GetStocks()
	core.InsertStocks(stocks)
}
