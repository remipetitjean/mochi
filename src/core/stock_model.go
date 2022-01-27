package core

import (
	"fmt"

	"gorm.io/gorm/clause"
)

type Stock struct {
	Symbol    string       `json:"symbol" gorm:"primaryKey;unique;index"` // Refers to the symbol represented in Nasdaq Integrated symbology (INET)
	Exchange  string       `json:"exchange"`                              // Refers to Exchange using IEX Supported Exchanges list
	Name      string       `json:"name"`                                  // Refers to the name of the company or security
	Date      string       `json:"date"`                                  //	Refers to the date the symbol reference data was generated
	IsEnabled bool         `json:"isEnabled"`                             // Will be true if the symbol is enabled for trading on IEX
	Type      string       `json:"type"`                                  // Refers to the common issue type
	Region    string       `json:"region"`                                // Refers to the country code for the symbol using ISO 3166-1 alpha-2
	Currency  string       `json:"currency"`                              // Refers to the currency the symbol is traded in using ISO 4217
	IexId     string       `json:"iexId"`                                 // Unique ID applied by IEX to track securities through symbol changes
	Figi      string       `json:"figi"`                                  // OpenFIGI id for the security if available
	Cik       string       `json:"cik"`                                   // CIK number for the security if available
	Dummy     bool         `json:"dummy" gorm:"default:false"`
	Prices    []StockPrice `json:"prices" gorm:"foreignKey:Symbol;references:Symbol"`
}

func InsertStock(stock Stock) {
	fmt.Printf("Inserting stock %s\n", stock.Symbol)
	DB.Clauses(clause.OnConflict{
		UpdateAll: true,
	}).Create(&stock)
}

func InsertMissingStocks(stock_prices []StockPrice) {
	fmt.Println("Inserting missing symbols")
	symbols := make([]Stock, len(stock_prices))
	for i, price := range stock_prices {
		symbols[i] = Stock{Symbol: price.Symbol, Dummy: true}
	}
	DB.Clauses(clause.OnConflict{
		DoNothing: true,
	}).CreateInBatches(&symbols, 100)

}

func InsertStocks(stocks []Stock) {
	fmt.Printf("Inserting %d stocks into database\n", len(stocks))
	DB.Clauses(clause.OnConflict{
		UpdateAll: true,
	}).CreateInBatches(&stocks, 100)
}
