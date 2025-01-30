create schema blog

create table blog.post
(
    id    serial
        constraint post_pk
            primary key,
    title bpchar                   not null,
    text  text                     not null,
    date  timestamp with time zone not null
);

alter table blog.post
    owner to root;

