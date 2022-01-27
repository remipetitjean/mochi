package main

import (
	"shyrio.com/mochi/core"
	libs "shyrio.com/mochi/libs/iex"
)

func main() {
	core.ConnectDatabase()
	stocks := libs.GetStocks()
	core.InsertStocks(stocks)
}
