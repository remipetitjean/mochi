package routers

import (
	"github.com/gin-gonic/gin"
	controllers "shyrio.com/mochi/controllers"
)

func AddStockRoutes(router *gin.Engine) {
	stock_router := router.Group("/stock")
	stock_router.GET("/", controllers.GetStocks)
	stock_router.GET("/:symbol", controllers.GetStock)
	stock_router.GET("/:symbol/price", controllers.GetStockPrices)
}
