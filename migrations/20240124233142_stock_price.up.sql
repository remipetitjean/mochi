CREATE TABLE stock_price (
    symbol varchar REFERENCES stock (symbol) not null,
    exchange varchar(5) REFERENCES exchange (code) not null,
    currency char(3) REFERENCES currency (code) not null,
    eod date not null,
    open double precision not null,
    high double precision not null,
    low double precision not null,
    close double precision not null,
    volume int not null
);

-- https://api.twelvedata.com/time_series?symbol=aapl&interval=1day&apikey=16ebf3860688468b9cdab89899669b30&dp=11
