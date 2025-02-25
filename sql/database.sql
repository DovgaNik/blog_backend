create schema blog

    create table blog.language
    (
        id   serial
            constraint language_pk
                primary key,
        name varchar(16) not null
    );

alter table blog.language
    owner to admin;

create table blog.posts
(
    id       serial
        constraint posts_pk
            primary key,
    title    bpchar                   not null,
    text     text                     not null,
    date     timestamp with time zone not null,
    language integer                  not null
        constraint language_fk
            references blog.language
            on delete restrict
);

alter table blog.posts
    owner to admin;