package main

import (
	"shyrio.com/position-service/core"
)

func main() {
	core.ConnectDatabase()
	core.DB.AutoMigrate(&core.Stock{})
	core.DB.AutoMigrate(&core.StockPrice{})
}
