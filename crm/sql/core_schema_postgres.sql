create table account
(
    account_id    bigserial                not null
        constraint account_pk
            primary key,
    name          varchar(64)              not null,
    password_hash varchar(64)              not null,
    roles         varchar(64)              not null,
    policies      varchar(64)              not null,
    created       timestamp with time zone not null,
    allow_login   boolean                  not null
);

create unique index account_name_uindex
    on account (name);
