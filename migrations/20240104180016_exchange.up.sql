CREATE TABLE exchange (
    code varchar(5) primary key not null,
    name varchar not null,
    country_code char(2) not null REFERENCES country (code) not null,
    timezone_code varchar not null REFERENCES timezone (code) not null
);
