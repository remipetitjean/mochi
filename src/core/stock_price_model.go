package core

import (
	"fmt"

	"gorm.io/gorm/clause"
)

type StockPrice struct {
	Symbol               string  `json:"symbol" gorm:"primaryKey"` // Associated symbol or ticker
	Date                 string  `json:"date" gorm:"primaryKey"`
	Close                float32 `json:"close"`                // Adjusted data for historical dates. Split adjusted only
	High                 float32 `json:"high"`                 // Adjusted data for historical dates. Split adjusted only
	Low                  float32 `json:"low"`                  // Adjusted data for historical dates. Split adjusted only
	Open                 float32 `json:"open"`                 // Adjusted data for historical dates. Split adjusted only
	Volume               float32 `json:"volume"`               // Adjusted data for historical dates. Split adjusted only
	ChangeOverTime       float32 `json:"changeOverTime"`       // Percent change of each interval relative to first value. Useful for comparing multiple stocks
	MarketChangeOverTime float32 `json:"marketChangeOverTime"` // Percent change of each interval relative to first value. 15 minute delayed consolidated data
	UnadjustedOpen       float32 `json:"uOpen"`                // Unadjusted data for historical dates
	UnadjustedClose      float32 `json:"uClose"`               // Unadjusted data for historical dates
	UnadjustedHigh       float32 `json:"uHigh"`                // Unadjusted data for historical dates
	UnadjustedLow        float32 `json:"uLow"`                 // Unadjusted data for historical dates
	UnadjustedVolume     float32 `json:"uVolume"`              // Unadjusted data for historical dates
	FullyAdjustedOpen    float32 `json:"fOpen"`                // Fully adjusted for historical dates
	FullyAdjustedClose   float32 `json:"fClose"`               // Fully adjusted for historical dates
	FullyAdjustedHigh    float32 `json:"fHigh"`                // Fully adjusted for historical dates
	FullyAdjustedLow     float32 `json:"fLow"`                 // Fully adjusted for historical dates
	FullyAdjustedVolume  float32 `json:"fVolume"`              // Fully adjusted for historical dates
	Change               float32 `json:"change"`               // Change from previous trading day
	ChangePercent        float32 `json:"changePercent"`        // Change persent from previous trading day
}

func InsertStockPrice(stock_price StockPrice) {
	fmt.Printf("Inserting stock price [%s|%s]\n", stock_price.Symbol, stock_price.Date)
	DB.Clauses(clause.OnConflict{
		UpdateAll: true,
	}).Create(&stock_price)
}

func InsertStockPrices(stock_prices []StockPrice) {
	fmt.Printf("Inserting %d stocks into database\n", len(stock_prices))
	DB.Clauses(clause.OnConflict{
		UpdateAll: true,
	}).CreateInBatches(&stock_prices, 100)
}
