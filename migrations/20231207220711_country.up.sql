CREATE TABLE region (
    code varchar(5) primary key not null,
    region varchar(9) not null,
    sub_region varchar(31) null,
    intermediate_region varchar(15) null
);

CREATE TABLE country (
    code char(2) primary key not null,
    code_3 char(3) not null,
    name varchar not null,
    region varchar(5) REFERENCES region (code) not null
);
