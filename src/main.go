package main

import (
	"github.com/gin-gonic/gin"
	"shyrio.com/mochi/core"
	"shyrio.com/mochi/routers"
)

var router *gin.Engine

func main() {
	core.ConnectDatabase()

	router = gin.New()
	routers.AddRoutes(router)
	router.Run()
}
