package libs

import (
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"

	"shyrio.com/mochi/core"
)

func GetPreviousDayStockPrices() []core.StockPrice {
	// https://iexcloud.io/docs/api/#previous-day-price
	endpoint := fmt.Sprintf("%s/stock/market/previous?token=%s", IexUrl, IexToken)

	resp, err := http.Get(endpoint)
	if err != nil {
		log.Fatal(err)
	}

	bs, err := io.ReadAll(resp.Body)
	resp.Body.Close()
	if err != nil {
		log.Fatal(err)
	}

	var prices []core.StockPrice
	if err := json.Unmarshal(bs, &prices); err != nil {
		log.Fatal(err)
	}

	return prices
}

func GetStockPricesSinceInception(symbol string) []core.StockPrice {
	//https://iexcloud.io/docs/api/#historical-prices

	endpoint := fmt.Sprintf("%s/stock/%s/chart/max?token=%s", IexUrl, symbol, IexToken)

	resp, err := http.Get(endpoint)
	if err != nil {
		log.Fatal(err)
	}

	bs, err := io.ReadAll(resp.Body)
	resp.Body.Close()
	if err != nil {
		log.Fatal(err)
	}

	var prices []core.StockPrice
	if err := json.Unmarshal(bs, &prices); err != nil {
		fmt.Printf("Failed unmarshalling '%s'", bs)
		log.Fatal(err)
	}

	return prices
}
