CREATE TABLE stock_exchange (
    symbol varchar REFERENCES stock (symbol) not null,
    exchange varchar(5) REFERENCES exchange (code) not null,
    currency char(3) REFERENCES currency (code) not null,
    is_default boolean default false,
    primary key (symbol, exchange)
);
