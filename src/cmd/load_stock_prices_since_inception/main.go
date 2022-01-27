package main

import (
	"flag"
	"os"

	"shyrio.com/mochi/core"
	libs "shyrio.com/mochi/libs/iex"
)

func main() {
	symbol := flag.String("symbol", "", "Symbol")
	flag.Parse()
	if *symbol == "" {
		os.Exit(1)
	}

	core.ConnectDatabase()
	stock_prices := libs.GetStockPricesSinceInception(*symbol)
	core.InsertStockPrices(stock_prices)
}
