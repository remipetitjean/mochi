CREATE TYPE globaltype AS ENUM (
    'Basic',
    'Level A',
    'Level B',
    'Level C'
);

CREATE TYPE plantype AS ENUM (
    'Basic',
    'Grow',
    'Pro',
    'Enterprise'
);

CREATE TABLE stock_td_plan (
    symbol varchar primary key REFERENCES stock (symbol) not null,
    global globaltype not null,
    plan plantype not null
);
