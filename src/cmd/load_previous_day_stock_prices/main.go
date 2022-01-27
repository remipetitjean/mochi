package main

import (
	"shyrio.com/position-service/core"
	libs "shyrio.com/position-service/libs/iex"
)

func main() {
	core.ConnectDatabase()
	stock_prices := libs.GetPreviousDayStockPrices()
	core.InsertMissingStocks(stock_prices)
	core.InsertStockPrices(stock_prices)
}
