CREATE TABLE students
(
    id                  UUID PRIMARY KEY   DEFAULT gen_random_uuid_v7(),
    name                text      NOT NULL,
    gender              int2,
    id_number           text      NOT NULL,
    date_of_birth       TIMESTAMP,
    school_name         text,
    grade               int2,
    is_pg               bool,
    description         text,
    family_type         text,
    family_members      int2,
    breadwinner         text,
    occupation          text,
    subsidy             text,
    address             text,
    home_ownership      int2,
    home_phone_number   text,
    mobile_phone_number text,
    chinese_book        text,
    english_book        text,
    math_book           text,
    science_book        text,
    social_studies_book text,
    line_id             text,
    comment             text,
    joined_at           TIMESTAMP NOT NULL,
    created_at          TIMESTAMP NOT NULL DEFAULT timezone('utc', now()),
    updated_at          TIMESTAMP NOT NULL DEFAULT timezone('utc', now()),
    deleted_at          TIMESTAMP
);