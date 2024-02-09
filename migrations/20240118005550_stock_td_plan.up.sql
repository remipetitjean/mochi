CREATE TABLE stock_td_plan (
    symbol varchar primary key REFERENCES stock (symbol) not null,
    global varchar not null,
    plan varchar not null
);
