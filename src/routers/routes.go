package routers

import (
	"github.com/gin-gonic/gin"
)

func AddRoutes(router *gin.Engine) {
	AddPingRoutes(router)
	AddStockRoutes(router)
}
