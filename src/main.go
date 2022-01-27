package main

import (
	"github.com/gin-gonic/gin"
	"shyrio.com/position-service/core"
	"shyrio.com/position-service/routers"
)

var router *gin.Engine

func main() {
	core.ConnectDatabase()

	router = gin.New()
	routers.AddRoutes(router)
	router.Run()
}
