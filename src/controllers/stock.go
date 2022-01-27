package contollers

import (
	"net/http"

	"github.com/gin-gonic/gin"
	"shyrio.com/mochi/core"
)

func GetStocks(c *gin.Context) {
	var stocks []core.Stock
	core.DB.Find(&stocks)
	c.JSON(http.StatusOK, stocks)
}

func GetStock(c *gin.Context) {
	symbol := c.Param("symbol")
	var stock core.Stock
	core.DB.Where(&core.Stock{Symbol: symbol}).First(&stock)
	c.JSON(http.StatusOK, stock)
}

func GetStockPrices(c *gin.Context) {
	symbol := c.Param("symbol")
	var stock_prices []core.StockPrice
	core.DB.Where(&core.StockPrice{Symbol: symbol}).Find(&stock_prices)
	c.JSON(http.StatusOK, stock_prices)
}
