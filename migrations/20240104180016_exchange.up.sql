CREATE TABLE exchange (
    code char(4) primary key not null,
    name varchar not null,
    country_code char(2) REFERENCES country (code) not null,
    timezone varchar not null
);
