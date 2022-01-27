package main

import (
	"shyrio.com/mochi/core"
	libs "shyrio.com/mochi/libs/iex"
)

func main() {
	core.ConnectDatabase()
	stock_prices := libs.GetPreviousDayStockPrices()
	core.InsertMissingStocks(stock_prices)
	core.InsertStockPrices(stock_prices)
}
