package core

import "os"

var DbUser = os.Getenv("DB_USER")

var DbPassword = os.Getenv("DB_PASSWORD")

var DbName = os.Getenv("DB_NAME")

var DbHost = os.Getenv("DB_SERVICE_HOST")

var DbPort = os.Getenv("DB_SERVICE_PORT")
