CREATE TYPE timezonetype AS ENUM ('Canonical', 'Link');

CREATE TABLE timezone (
    code varchar primary key not null,
    type timezonetype not null,
    utc_offset varchar not null,
    utc_dst_offset varchar not null,
    tz_abbreviation varchar not null,
    tz_dst_abbreviation varchar not null,
    is_active bool not null
);
