package libs

import (
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"

	"shyrio.com/mochi/core"
)

func GetStocks() []core.Stock {
	// https://iexcloud.io/docs/api/#stocks

	endpoint := fmt.Sprintf("%s/ref-data/symbols?token=%s", IexUrl, IexToken)

	resp, err := http.Get(endpoint)
	if err != nil {
		log.Fatal(err)
	}

	bs, err := io.ReadAll(resp.Body)
	resp.Body.Close()
	if err != nil {
		log.Fatal(err)
	}

	var stocks []core.Stock
	if err := json.Unmarshal(bs, &stocks); err != nil {
		log.Fatal(err)
	}

	return stocks
}
