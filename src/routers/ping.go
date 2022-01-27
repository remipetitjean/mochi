package routers

import (
	"github.com/gin-gonic/gin"
	controllers "shyrio.com/mochi/controllers"
)

func AddPingRoutes(router *gin.Engine) {
	ping_router := router.Group("/ping")
	ping_router.GET("/", controllers.Ping)
}
