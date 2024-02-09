CREATE TABLE stock_exchange (
    symbol varchar REFERENCES stock (symbol) not null,
    exchange varchar(5) REFERENCES exchange (code) not null,
    currency char(3) REFERENCES currency (code) not null,
    primary key (symbol, exchange)
);
