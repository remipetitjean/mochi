CREATE TYPE stocktype AS ENUM (
    'American Depositary Receipt',
    'Closed-end Fund',
    'Common Stock',
    'Depositary Receipt',
    'ETF',
    'Exchange-Traded Note',
    'Global Depositary Receipt',
    'Limited Partnership',
    'Mutual Fund',
    'Preferred Stock',
    'REIT',
    'Right',
    'Structured Product',
    'Trust',
    'Unit',
    'Warrant'
);

CREATE TABLE stock (
    symbol varchar primary key not null,
    name varchar not null,
    currency char(2) REFERENCES currency (code) not null,
    exchange varchar(5) REFERENCES exchange (code) not null,
    country char(2) REFERENCES country (code) not null,
    stock_type stocktype not null
);
